#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! MGGA kernel translations batch 35.

pub mod mgga_c_cc;
pub mod mgga_k_lk;
// mgga_x_2d_prp10 deferred: requires xc_bessel_I0/I1 (Bessel functions)
// pub mod mgga_x_2d_prp10;
pub mod mgga_x_jk;
pub mod mgga_x_mvs;
pub mod mgga_x_tb09;
pub mod mgga_xc_cc06;
