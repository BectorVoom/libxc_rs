//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1003 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3418;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3419;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3420;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3421;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3422;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3423;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3424;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3425;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3426;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1003<F: Float>(t52033: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F, t63377: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52065: F, t63393: F, t63396: F, t63399: F, t63469: F, t63471: F, t41330: F, t41332: F, t63474: F, t63476: F, t63478: F, t63480: F, t63482: F, t63485: F, t63488: F, t63491: F, t63494: F, t63497: F, t63500: F, t63503: F, t63505: F, t52126: F, t52128: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63519: F, t63522: F, t63525: F, t63528: F, t63531: F, t63533: F, t63536: F, t63538: F, t41441: F, t63462: F, t63464: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t63554: F, t63557: F, t63560: F, t63563: F, t63566: F, t63568: F, t2942: F, t6152: F, t11409: F, t11461: F, t11554: F, t15249: F, t15259: F, t15284: F, t15287: F, t15350: F, t15406: F, t15413: F, t19269: F, t19290: F, t19294: F, t19297: F, t19300: F, t2944: F, t2945: F, t2968: F, t2970: F, t41779: F, t41788: F, t41799: F, t4690: F, t4712: F, t52370: F, t52440: F, t52459: F, t52637: F, t52837: F, t6158: F, t6177: F, t63679: F, t63916: F, t64109: F, t64197: F, t64212: F, t64228: F, t946: F, t954: F, t972: F, t52163: F, t52482: F, t934: F, t15390: F, t52514: F, t19056: F, t2919: F, t2923: F, t6104: F, t2927: F, t1610: F, t52214: F, t15416: F, t4632: F, t15475: F, t4590: F, t41880: F, t6110: F, t41549: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F, t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t64244 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3418::<F>(t52033, t63336, t63338, t63340, t63342, t63346, t63351, t63355, t63359, t63361, t63366, t63369, t63371, t63374, t63377);
        let t64261 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3419::<F>(t52035, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52065, t63393, t63396, t63399, t63469, t63471);
        let t64277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3420::<F>(t41330, t41332, t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491, t63494, t63497, t63500, t63503, t63505);
        let t64294 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3421::<F>(t52126, t52128, t63447, t63451, t63453, t63457, t63459, t63519, t63522, t63525, t63528, t63531, t63533, t63536, t63538);
        let t64310 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3422::<F>(t41441, t63462, t63464, t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557, t63560, t63563, t63566, t63568);
        let t64324 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3423::<F>(t2942, t6152, t11409, t11461, t11554, t15249, t15259, t15284, t15287, t15350, t15406, t15413, t19269, t19290, t19294, t19297, t19300, t2944, t2945, t2968, t2970, t41779, t41788, t41799, t4690, t4712, t52370, t52440, t52459, t52637, t52837, t6158, t6177, t63679, t63916, t64109, t64197, t64212, t64228, t64244, t64261, t64277, t64294, t64310, t946, t954, t972);
        let (t64327, t64329, t64335, t64338, t64340) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3424::<F>(t52163, t52482, t934, t15390, t52514, t19056, t2919, t2923, t6104, t2927, t1610, t52214);
        let (t64342, t64344, t64346, t64358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3425::<F>(t15416, t4632, t15475, t4590, t41880, t6110, t41549, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t64372 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3426::<F>(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t64386 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3427::<F>(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
    (t64324, t64327, t64329, t64335, t64338, t64340, t64342, t64344, t64346, t64358, t64372, t64386)
}
