//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2441;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta654(t11727: f64, t3106: f64, t3223: f64, t3230: f64, t11817: f64, t3224: f64, t1024: f64, t11961: f64, t3042: f64, t3056: f64, t225: f64, t11274: f64, t12009: f64, t11277: f64, t11916: f64, t11246: f64, t11251: f64, t3172: f64, t11648: f64, t3169: f64, t1062: f64, t11782: f64, t10356: f64, t11150: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42338, t42340, t42346, t42355, t42358, t42359, t42369) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2441(t11727, t3106, t3223, t3230, t11817, t3224, t1024, t11961, t3042, t3056, t225, t11274, t12009);
        let (t42374, t42377, t42383, t42391, t42397) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2442(t11277, t11916, t11246, t11251, t3172, t11648, t3169, t1062, t11782, t10356, t11150, t357);
    (t42338, t42340, t42346, t42355, t42358, t42359, t42369, t42374, t42377, t42383, t42391, t42397)
}
