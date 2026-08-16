//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta47 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk324;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk325;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk326;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta47(t884: f64, t908: f64, t136: f64, t886: f64, t897: f64, t899: f64, t902: f64, t907: f64, t290: f64, t893: f64, t880: f64, t307: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t909, t910, t912) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk324(t884, t908, t136, t886, t897, t899, t902, t907);
        let t913 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk325(t290);
        let (t914, t916, t917, t919, t922, t923) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk326(t912, t913, t893, t880, t886, t307);
        let t924 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk327(t302, t923);
    (t909, t910, t912, t913, t914, t916, t917, t919, t922, t923, t924)
}
