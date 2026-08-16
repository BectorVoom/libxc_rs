//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1048;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta289(t136: f64, t860: f64, t2457: f64, t2710: f64, t10069: f64, t2786: f64, t10073: f64, t10111: f64, t22: f64, t870: f64, t10115: f64, t253: f64, t2777: f64, t2789: f64, t2439: f64, t2435: f64, t2790: f64, t2778: f64, t9303: f64, t871: f64, t9292: f64, t251: f64, t9646: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10916, t10923, t10925, t10939, t10948) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1048(t136, t860, t2457, t2710, t10069, t2786, t10073, t10111, t22, t870, t10115, t253);
        let (t10964, t10966, t10969, t10971, t10981, t10982) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1049(t2777, t2789, t2439, t2435, t2790, t2778, t9303, t871, t9292, t251, t9646, t22, t780);
    (t10916, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t10981, t10982)
}
