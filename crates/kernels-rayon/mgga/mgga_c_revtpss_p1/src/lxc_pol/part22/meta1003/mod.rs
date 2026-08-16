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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1003(t52033: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64, t63377: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52065: f64, t63393: f64, t63396: f64, t63399: f64, t63469: f64, t63471: f64, t41330: f64, t41332: f64, t63474: f64, t63476: f64, t63478: f64, t63480: f64, t63482: f64, t63485: f64, t63488: f64, t63491: f64, t63494: f64, t63497: f64, t63500: f64, t63503: f64, t63505: f64, t52126: f64, t52128: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63519: f64, t63522: f64, t63525: f64, t63528: f64, t63531: f64, t63533: f64, t63536: f64, t63538: f64, t41441: f64, t63462: f64, t63464: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t63554: f64, t63557: f64, t63560: f64, t63563: f64, t63566: f64, t63568: f64, t2942: f64, t6152: f64, t11409: f64, t11461: f64, t11554: f64, t15249: f64, t15259: f64, t15284: f64, t15287: f64, t15350: f64, t15406: f64, t15413: f64, t19269: f64, t19290: f64, t19294: f64, t19297: f64, t19300: f64, t2944: f64, t2945: f64, t2968: f64, t2970: f64, t41779: f64, t41788: f64, t41799: f64, t4690: f64, t4712: f64, t52370: f64, t52440: f64, t52459: f64, t52637: f64, t52837: f64, t6158: f64, t6177: f64, t63679: f64, t63916: f64, t64109: f64, t64197: f64, t64212: f64, t64228: f64, t946: f64, t954: f64, t972: f64, t52163: f64, t52482: f64, t934: f64, t15390: f64, t52514: f64, t19056: f64, t2919: f64, t2923: f64, t6104: f64, t2927: f64, t1610: f64, t52214: f64, t15416: f64, t4632: f64, t15475: f64, t4590: f64, t41880: f64, t6110: f64, t41549: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64, t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t64244 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3418(t52033, t63336, t63338, t63340, t63342, t63346, t63351, t63355, t63359, t63361, t63366, t63369, t63371, t63374, t63377);
        let t64261 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3419(t52035, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52065, t63393, t63396, t63399, t63469, t63471);
        let t64277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3420(t41330, t41332, t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491, t63494, t63497, t63500, t63503, t63505);
        let t64294 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3421(t52126, t52128, t63447, t63451, t63453, t63457, t63459, t63519, t63522, t63525, t63528, t63531, t63533, t63536, t63538);
        let t64310 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3422(t41441, t63462, t63464, t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557, t63560, t63563, t63566, t63568);
        let t64324 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3423(t2942, t6152, t11409, t11461, t11554, t15249, t15259, t15284, t15287, t15350, t15406, t15413, t19269, t19290, t19294, t19297, t19300, t2944, t2945, t2968, t2970, t41779, t41788, t41799, t4690, t4712, t52370, t52440, t52459, t52637, t52837, t6158, t6177, t63679, t63916, t64109, t64197, t64212, t64228, t64244, t64261, t64277, t64294, t64310, t946, t954, t972);
        let (t64327, t64329, t64335, t64338, t64340) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3424(t52163, t52482, t934, t15390, t52514, t19056, t2919, t2923, t6104, t2927, t1610, t52214);
        let (t64342, t64344, t64346, t64358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3425(t15416, t4632, t15475, t4590, t41880, t6110, t41549, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t64372 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3426(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t64386 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3427(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
    (t64324, t64327, t64329, t64335, t64338, t64340, t64342, t64344, t64346, t64358, t64372, t64386)
}
