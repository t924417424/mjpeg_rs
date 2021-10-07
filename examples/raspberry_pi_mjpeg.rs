use std::{sync::Arc, thread};

use mjpeg_rs::MJpeg;
// use rascam::*;
// 使用rascam调用摄像机
fn main() {
    // let info = info().unwrap();
    // if info.cameras.len() < 1 {
    //     println!("Found 0 cameras. Exiting");
    //     // note that this doesn't run destructors
    //     ::std::process::exit(1);
    // }
    // println!("{}", info);
    // let info = &info.cameras[0];
    // let mut camera = SimpleCamera::new(info.clone()).unwrap();
    // let settings = CameraSettings {
    //     encoding: MMAL_ENCODING_JPEG,
    //     width: 600,
    //     height: 600,
    //     iso: ISO_AUTO,
    //     zero_copy: false,
    //     use_encoder: true,
    // };
    // camera.configure(settings);
    // camera.activate().unwrap();
    let m = Arc::new(MJpeg::new());
    let mrc = m.clone();
    thread::spawn(move || mrc.run("0.0.0.0:8088").unwrap());
    // loop {
    //     let b = camera.take_one().unwrap();
    //     m.update_jpeg(b).unwrap();
    // }
}