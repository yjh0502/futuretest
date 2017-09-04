extern crate futures;
extern crate tokio_core;
extern crate hyper;

use futures::*;
use tokio_core::reactor::*;
use hyper::Client;
use std::time;

fn send_request(remote: Remote) -> Result<hyper::Response, futures::Canceled> {
    let (tx, rx) = futures::oneshot();
    remote.spawn(|handle| {
        let client = Client::new(&handle);
        let uri = "http://rust-lang.org".parse().unwrap();
        client
            .get(uri)
            .map_err(|_err| ())
            .and_then(|resp| {
                println!("resp: {}", resp.version());
                tx.send(resp).unwrap();
                Ok(())
            })
            .or_else(|_err| Err(()))
    });
    rx.wait()
}

#[allow(unused)]
fn fetch_sync() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let remote = handle.remote().clone();

    std::thread::spawn(move || send_request(remote));

    let t = time::SystemTime::now();
    let deadline = time::Duration::new(2, 0);
    while t.elapsed().unwrap() < deadline {
        core.turn(Some(time::Duration::new(1, 0)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        fetch_sync();
    }
}
