//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2630;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2631;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2632;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2633;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta752<F: Float>(t30: F, t1868: F, t9940: F, t5577: F, t588: F, t1344: F, t13687: F, t13690: F, t1468: F, t2: F, t22: F, t3874: F, t46310: F, t48165: F, t48168: F, t48174: F, t48177: F, t5574: F, t580: F, t605: F, t9336: F, t9344: F, t9605: F, zeta_threshold: F, t33: F, t5585: F, t1113: F, t1348: F, t13701: F, t13704: F, t1711: F, t3881: F, t46328: F, t48192: F, t48195: F, t48201: F, t48204: F, t5582: F, t9351: F, t9357: F, t9617: F, t4010: F, t5591: F, t1353: F, t13716: F, t13892: F, t13902: F, t13910: F, t13911: F, t13914: F, t13917: F, t1392: F, t1394: F, t1395: F, t1412: F, t1879: F, t3829: F, t3889: F, t4050: F, t539: F, t5644: F, t5650: F, t5651: F, t9628: F, t9872: F, t13768: F, t13907: F, t1877: F, t22229: F, t225: F, t4045: F, t4053: F, t48220: F, t48245: F, t48257: F, t48272: F, t48289: F, t48309: F, t48321: F, t48337: F, t541: F, t543: F, t5652: F, t5655: F, t73: F, t9400: F, t9881: F, t9884: F, t9887: F, t9984: F, t13921: F, t221: F, t4018: F, t4019: F, t2661: F, t3924: F, t3992: F, t5608: F, t1882: F, t9956: F) -> (F, F, F, F, F, F, F, F) {
        let (t48347, t48396) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2630::<F>(t30, t1868, t9940, t5577, t588, t1344, t13687, t13690, t1468, t2, t22, t3874, t46310, t48165, t48168, t48174, t48177, t5574, t580, t605, t9336, t9344, t9605, zeta_threshold);
        let t48419 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2631::<F>(t33, t5585, t588, t1113, t1348, t13701, t13704, t1711, t2, t22, t3881, t46328, t48192, t48195, t48201, t48204, t5582, t580, t9351, t9357, t9617, zeta_threshold);
        let (t48421, t48432, t48436) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2632::<F>(t48396, t48419, t4010, t5591, t1353, t13716, t13892, t13902, t13910, t13911, t13914, t13917, t1392, t1394, t1395, t1412, t1879, t3829, t3889, t4050, t539, t5644, t5650, t5651, t9628, t9872);
        let t48438 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2633::<F>(t13768, t13902, t13907, t1877, t22229, t225, t4045, t4053, t48220, t48245, t48257, t48272, t48289, t48309, t48321, t48337, t48347, t48436, t541, t543, t5644, t5650, t5652, t5655, t73, t9400, t9881, t9884, t9887, t9984);
        let (t48445, t48449, t48453, t48458) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2634::<F>(t13921, t221, t4018, t4019, t2661, t3924, t3992, t5651, t5608, t1882, t4010, t9956);
    (t48347, t48421, t48432, t48438, t48445, t48449, t48453, t48458)
}
