//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1004 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3428;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3429;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3430;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3431;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3432;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3433;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3434;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1004<F: Float>(t41330: F, t41332: F, t52047: F, t52049: F, t52051: F, t63399: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63462: F, t63464: F, t291: F, t64358: F, t64372: F, t64386: F, t41908: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F, t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F, t2875: F, t2924: F, t6142: F, t15380: F, t52645: F, t19330: F, t2918: F, t11385: F, t11387: F, t6141: F, t15098: F, t15421: F, t11466: F, t2988: F, t3012: F, t311: F, t41238: F, t41658: F, t6189: F, t6190: F, t6206: F, t63892: F, t64327: F, t64329: F, t64335: F, t64338: F, t64340: F, t64342: F, t64344: F, t64346: F, t300: F, t63975: F, t64023: F, t64068: F, t64101: F, t64146: F, t64152: F, t64324: F, t18898: F, t3015: F, t981: F) -> (F, F, F, F, F, F, F, F) {
        let t64400 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3428::<F>(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t64404, t64416) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3429::<F>(t291, t64358, t64372, t64386, t64400, t41908, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t64430 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3430::<F>(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t64444 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3431::<F>(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t64458 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3432::<F>(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t64465, t64467, t64471, t64475, t64483) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3433::<F>(t2875, t2924, t6142, t15380, t52645, t19330, t2918, t11385, t11387, t6141, t15098, t15421);
        let t64484 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3434::<F>(t11466, t2988, t3012, t311, t41238, t41658, t6189, t6190, t6206, t63892, t64327, t64329, t64335, t64338, t64340, t64342, t64344, t64346, t64404, t64416, t64430, t64444, t64458, t64465, t64467, t64471, t64475, t64483);
        let (t64488, t64491) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3435::<F>(t300, t63975, t64023, t64068, t64101, t64146, t64152, t64324, t64484, t18898, t3015, t981);
    (t64404, t64465, t64467, t64471, t64475, t64483, t64488, t64491)
}
