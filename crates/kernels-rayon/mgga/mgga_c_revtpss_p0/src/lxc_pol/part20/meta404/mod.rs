//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1495;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1496;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta404(t11817: f64, t3224: f64, t1024: f64, t11961: f64, t3042: f64, t3056: f64, t225: f64, t366: f64, t11274: f64, t12009: f64, t11273: f64, t11998: f64, t11277: f64, t11916: f64, t11246: f64, t11251: f64, t3172: f64, t1025: f64, t1028: f64, t11659: f64, t11811: f64, t11994: f64, t12026: f64, t15963: f64, t3092: f64, t3164: f64, t3208: f64, t371: f64, t372: f64, t373: f64, t42097: f64, t4899: f64, t11648: f64, t3169: f64, t3133: f64, t1062: f64, t11782: f64, t10356: f64, t11150: f64, t357: f64, t11853: f64, t828: f64, t3229: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42346, t42355, t42358, t42359, t42360, t42369, t42371) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1495(t11817, t3224, t1024, t11961, t3042, t3056, t225, t366, t11274, t12009, t11273, t11998);
        let t42379 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1496(t11277, t11916, t11246, t11251, t3172, t1025, t1028, t11659, t11811, t11994, t12026, t15963, t3092, t3164, t3208, t3224, t371, t372, t373, t42097, t42346, t42355, t42360, t42369, t42371, t4899);
        let (t42383, t42385, t42386, t42391, t42397, t42410, t42415) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1497(t11648, t3169, t3133, t373, t1062, t11782, t10356, t11150, t357, t11853, t828, t3229, t360);
    (t42358, t42359, t42379, t42383, t42385, t42386, t42391, t42397, t42410, t42415)
}
