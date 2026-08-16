//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2630;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2631;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2632;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2633;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta752(t30: f64, t1868: f64, t9940: f64, t5577: f64, t588: f64, t1344: f64, t13687: f64, t13690: f64, t1468: f64, t2: f64, t22: f64, t3874: f64, t46310: f64, t48165: f64, t48168: f64, t48174: f64, t48177: f64, t5574: f64, t580: f64, t605: f64, t9336: f64, t9344: f64, t9605: f64, zeta_threshold: f64, t33: f64, t5585: f64, t1113: f64, t1348: f64, t13701: f64, t13704: f64, t1711: f64, t3881: f64, t46328: f64, t48192: f64, t48195: f64, t48201: f64, t48204: f64, t5582: f64, t9351: f64, t9357: f64, t9617: f64, t4010: f64, t5591: f64, t1353: f64, t13716: f64, t13892: f64, t13902: f64, t13910: f64, t13911: f64, t13914: f64, t13917: f64, t1392: f64, t1394: f64, t1395: f64, t1412: f64, t1879: f64, t3829: f64, t3889: f64, t4050: f64, t539: f64, t5644: f64, t5650: f64, t5651: f64, t9628: f64, t9872: f64, t13768: f64, t13907: f64, t1877: f64, t22229: f64, t225: f64, t4045: f64, t4053: f64, t48220: f64, t48245: f64, t48257: f64, t48272: f64, t48289: f64, t48309: f64, t48321: f64, t48337: f64, t541: f64, t543: f64, t5652: f64, t5655: f64, t73: f64, t9400: f64, t9881: f64, t9884: f64, t9887: f64, t9984: f64, t13921: f64, t221: f64, t4018: f64, t4019: f64, t2661: f64, t3924: f64, t3992: f64, t5608: f64, t1882: f64, t9956: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48347, t48396) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2630(t30, t1868, t9940, t5577, t588, t1344, t13687, t13690, t1468, t2, t22, t3874, t46310, t48165, t48168, t48174, t48177, t5574, t580, t605, t9336, t9344, t9605, zeta_threshold);
        let t48419 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2631(t33, t5585, t588, t1113, t1348, t13701, t13704, t1711, t2, t22, t3881, t46328, t48192, t48195, t48201, t48204, t5582, t580, t9351, t9357, t9617, zeta_threshold);
        let (t48421, t48432, t48436) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2632(t48396, t48419, t4010, t5591, t1353, t13716, t13892, t13902, t13910, t13911, t13914, t13917, t1392, t1394, t1395, t1412, t1879, t3829, t3889, t4050, t539, t5644, t5650, t5651, t9628, t9872);
        let t48438 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2633(t13768, t13902, t13907, t1877, t22229, t225, t4045, t4053, t48220, t48245, t48257, t48272, t48289, t48309, t48321, t48337, t48347, t48436, t541, t543, t5644, t5650, t5652, t5655, t73, t9400, t9881, t9884, t9887, t9984);
        let (t48445, t48449, t48453, t48458) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2634(t13921, t221, t4018, t4019, t2661, t3924, t3992, t5651, t5608, t1882, t4010, t9956);
    (t48347, t48421, t48432, t48438, t48445, t48449, t48453, t48458)
}
