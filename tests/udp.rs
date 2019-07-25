#![feature(async_await)]
use std::convert::TryFrom;
use std::net::SocketAddr;
use futures::executor;
use romio::UdpSocket;

const THE_WINTERS_TALE: &[u8] = b"
                    Each your doing,
    So singular in each particular,
    Crowns what you are doing in the present deed,
    That all your acts are queens.
";

async fn exchange(mut socket: UdpSocket) {
	let addr = socket.local_addr().unwrap();
	let mut buf = vec![0; THE_WINTERS_TALE.len()];

	socket.send_to(THE_WINTERS_TALE, &addr).await.unwrap();
	let (n, sender) = socket.recv_from(&mut buf).await.unwrap();
	assert_eq!(sender, addr);
	assert_eq!(&buf[..n], THE_WINTERS_TALE);
}

#[test]
fn socket_sends_and_receives() {
    drop(env_logger::try_init());
    let socket = UdpSocket::bind(&"127.0.0.1:0".parse().unwrap()).unwrap();
    executor::block_on(exchange(socket));
}

#[test]
fn socket_from_std() {
    drop(env_logger::try_init());
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let std_socket = std::net::UdpSocket::bind(&addr).unwrap();
    let socket = UdpSocket::try_from(std_socket).unwrap();
    executor::block_on(exchange(socket));
}