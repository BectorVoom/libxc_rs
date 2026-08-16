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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3428;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3429;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3430;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3431;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3432;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3433;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3434;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1004(t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63462: f64, t63464: f64, t291: f64, t64358: f64, t64372: f64, t64386: f64, t41908: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64, t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64, t2875: f64, t2924: f64, t6142: f64, t15380: f64, t52645: f64, t19330: f64, t2918: f64, t11385: f64, t11387: f64, t6141: f64, t15098: f64, t15421: f64, t11466: f64, t2988: f64, t3012: f64, t311: f64, t41238: f64, t41658: f64, t6189: f64, t6190: f64, t6206: f64, t63892: f64, t64327: f64, t64329: f64, t64335: f64, t64338: f64, t64340: f64, t64342: f64, t64344: f64, t64346: f64, t300: f64, t63975: f64, t64023: f64, t64068: f64, t64101: f64, t64146: f64, t64152: f64, t64324: f64, t18898: f64, t3015: f64, t981: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t64400 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3428(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t64404, t64416) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3429(t291, t64358, t64372, t64386, t64400, t41908, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t64430 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3430(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t64444 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3431(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t64458 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3432(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t64465, t64467, t64471, t64475, t64483) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3433(t2875, t2924, t6142, t15380, t52645, t19330, t2918, t11385, t11387, t6141, t15098, t15421);
        let t64484 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3434(t11466, t2988, t3012, t311, t41238, t41658, t6189, t6190, t6206, t63892, t64327, t64329, t64335, t64338, t64340, t64342, t64344, t64346, t64404, t64416, t64430, t64444, t64458, t64465, t64467, t64471, t64475, t64483);
        let (t64488, t64491) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3435(t300, t63975, t64023, t64068, t64101, t64146, t64152, t64324, t64484, t18898, t3015, t981);
    (t64404, t64465, t64467, t64471, t64475, t64483, t64488, t64491)
}
