//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1495;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1496;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta404<F: Float>(t11817: F, t3224: F, t1024: F, t11961: F, t3042: F, t3056: F, t225: F, t366: F, t11274: F, t12009: F, t11273: F, t11998: F, t11277: F, t11916: F, t11246: F, t11251: F, t3172: F, t1025: F, t1028: F, t11659: F, t11811: F, t11994: F, t12026: F, t15963: F, t3092: F, t3164: F, t3208: F, t371: F, t372: F, t373: F, t42097: F, t4899: F, t11648: F, t3169: F, t3133: F, t1062: F, t11782: F, t10356: F, t11150: F, t357: F, t11853: F, t828: F, t3229: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42346, t42355, t42358, t42359, t42360, t42369, t42371) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1495::<F>(t11817, t3224, t1024, t11961, t3042, t3056, t225, t366, t11274, t12009, t11273, t11998);
        let t42379 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1496::<F>(t11277, t11916, t11246, t11251, t3172, t1025, t1028, t11659, t11811, t11994, t12026, t15963, t3092, t3164, t3208, t3224, t371, t372, t373, t42097, t42346, t42355, t42360, t42369, t42371, t4899);
        let (t42383, t42385, t42386, t42391, t42397, t42410, t42415) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1497::<F>(t11648, t3169, t3133, t373, t1062, t11782, t10356, t11150, t357, t11853, t828, t3229, t360);
    (t42358, t42359, t42379, t42383, t42385, t42386, t42391, t42397, t42410, t42415)
}
