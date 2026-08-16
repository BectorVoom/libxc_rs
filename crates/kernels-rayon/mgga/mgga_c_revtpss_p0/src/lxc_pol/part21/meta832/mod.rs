//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta832 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3105;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3106;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3107;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3108;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3109;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3110;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3111;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3112;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3113;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta832(t12917: f64, t17396: f64, t1260: f64, t17289: f64, t13032: f64, t17524: f64, t17544: f64, t3708: f64, t13051: f64, t56730: f64, t12915: f64, t16771: f64, t247: f64, t5384: f64, t17763: f64, t3636: f64, t13085: f64, t5391: f64, t3568: f64, t606: f64, t12881: f64, t5381: f64, t127: f64, t12866: f64, t17650: f64, t5296: f64, t17861: f64, t3624: f64, t1042: f64, t1261: f64, t12646: f64, t12920: f64, t13055: f64, t13076: f64, t16714: f64, t1715: f64, t17202: f64, t17429: f64, t17729: f64, t17736: f64, t17786: f64, t3606: f64, t3626: f64, t3631: f64, t3714: f64, t44225: f64, t44421: f64, t5051: f64, t5293: f64, t53450: f64, t5386: f64, t12784: f64, t17451: f64, t17416: f64, t3647: f64, t11262: f64, t1247: f64, t5286: f64, t17501: f64, t3172: f64, t3711: f64, t13099: f64, t43776: f64, t12956: f64, t17217: f64, t12909: f64, t17395: f64, t12277: f64, t12777: f64, t12781: f64, t12822: f64, t12828: f64, t12836: f64, t12842: f64, t12847: f64, t12912: f64, t17235: f64, t17237: f64, t17448: f64, t21049: f64, t21306: f64, t44343: f64, t44346: f64, t5277: f64, t53474: f64, t17384: f64, t12772: f64, t17668: f64, t3625: f64, t17673: f64, t12910: f64, t12916: f64, t17460: f64, t17213: f64, t13069: f64, t5265: f64, t17332: f64, t17747: f64, t17749: f64, t11231: f64, t1266: f64, t12787: f64, t17261: f64, t17265: f64, t20921: f64, t44403: f64, t44405: f64, t44409: f64, t44411: f64, t44415: f64, t44418: f64, t45710: f64, t5330: f64, t5343: f64, t12732: f64, t1774: f64, t1222: f64, t16725: f64, t17471: f64, t16729: f64, t13017: f64, t5373: f64, t44546: f64, t5331: f64, t5334: f64, t17654: f64, t17657: f64, t56756: f64, t17528: f64, t44545: f64, t5230: f64, t12984: f64, t5327: f64, t12995: f64, t17438: f64, t1214: f64, t1235: f64, t1250: f64, t12629: f64, t17280: f64, t17475: f64, t17484: f64, t17534: f64, t17649: f64, t1808: f64, t20945: f64, t2251: f64, t3362: f64, t3613: f64, t3667: f64, t371: f64, t3718: f64, t372: f64, t3720: f64, t44427: f64, t44568: f64, t44609: f64, t44664: f64, t471: f64, t482: f64, t5351: f64, t5405: f64, t56179: f64, t56376: f64, t17303: f64, t12886: f64, t12627: f64, t489: f64, t17728: f64, t13011: f64, t5368: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57049, t57053, t57056, t57063, t57065, t57070) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3105(t12917, t17396, t1260, t17289, t13032, t17524, t17544, t3708, t13051, t56730, t12915, t16771, t247, t5384);
        let (t57075, t57077, t57083, t57094, t57098) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3106(t17763, t3636, t13085, t5391, t3568, t606, t12881, t5381, t127, t12866, t17650, t5296);
        let t57103 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3107(t17861, t3624, t1042, t1260, t1261, t12646, t12920, t13055, t13076, t16714, t1715, t17202, t17429, t17729, t17736, t17786, t3606, t3626, t3631, t3714, t44225, t44421, t5051, t5293, t53450, t5386, t57049, t57053, t57056, t57063, t57065, t57070, t57075, t57077, t57083, t57094, t57098);
        let (t57114, t57118, t57126, t57128, t57136) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3108(t12784, t17451, t17416, t3647, t11262, t1247, t5286, t17501, t3172, t3711, t13099, t43776);
        let t57150 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3109(t12956, t17217, t12909, t17395, t1042, t12277, t1261, t12777, t12781, t12822, t12828, t12836, t12842, t12847, t12912, t17235, t17237, t17448, t21049, t21306, t3647, t3711, t44343, t44346, t5277, t53450, t53474, t5381, t57114, t57118, t57126, t57128, t57136);
        let (t57164, t57167, t57170, t57173, t57176) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3110(t12784, t17384, t12772, t17668, t3625, t17673, t12910, t12916, t17460, t17213, t3172, t5384);
        let t57193 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3111(t13069, t5265, t1260, t17332, t12916, t17747, t17749, t11231, t1266, t12787, t12828, t17261, t17265, t17729, t20921, t44403, t44405, t44409, t44411, t44415, t44418, t45710, t5330, t5343, t5391, t57164, t57167, t57170, t57173, t57176);
        let (t57200, t57209, t57212, t57214, t57223, t57227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3112(t12732, t1774, t1222, t16725, t17471, t16729, t13017, t5373, t44546, t5331, t5334, t17654, t17657, t56756);
        let t57254 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3113(t13032, t17528, t247, t44545, t5230, t5384, t12984, t5327, t12995, t17438, t1214, t1222, t1235, t1250, t12629, t12866, t17280, t17475, t17484, t17534, t17649, t1808, t20945, t2251, t3362, t3613, t3667, t371, t3718, t372, t3720, t44427, t44568, t44609, t44664, t471, t482, t5351, t5405, t56179, t56376, t57200, t57209, t57212, t57214, t57223, t57227);
        let (t57257, t57258, t57264, t57265, t57271, t57273) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3114(t17303, t3667, t12886, t5381, t12627, t489, t17728, t13011, t5373, t1222, t5368, t697);
    (t57083, t57103, t57150, t57193, t57200, t57254, t57257, t57258, t57264, t57265, t57271, t57273)
}
