#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! MGGA kernel translations batch 26.

pub mod mgga_c_b88;
pub mod mgga_x_gdme;
// mgga_x_mbrxh_bg deferred: requires xc_mgga_x_br89_get_x (Brent's method root-finder)
// pub mod mgga_x_mbrxh_bg;
