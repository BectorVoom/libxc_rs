#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! MGGA kernel translations batch 24.

pub mod mgga_c_vsxc;
// mgga_x_mbr deferred: requires xc_mgga_x_br89_get_x (Brent's method root-finder)
// pub mod mgga_x_mbr;
