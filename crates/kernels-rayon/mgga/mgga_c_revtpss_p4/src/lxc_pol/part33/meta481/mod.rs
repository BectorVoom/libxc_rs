//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1758;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta481(t159: f64, t2698: f64, t218: f64, t816: f64, t228: f64, t7021: f64, t802: f64, t7043: f64, t826: f64, t2736: f64, t7082: f64, t72: f64, t686: f64, t7058: f64, t2453: f64, t7057: f64, t136: f64, t1958: f64, t2457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25273, t25276, t25277, t25278, t25282, t25284, t25295) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1758(t159, t2698, t218, t816, t228, t7021, t802, t7043, t826, t2736, t7082, t72);
        let (t25296, t25297, t25299, t25300, t25301) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1759(t25295, t686, t7058, t2453, t7057, t136, t1958, t2457);
    (t25273, t25276, t25277, t25278, t25282, t25284, t25295, t25296, t25297, t25299, t25300, t25301)
}
