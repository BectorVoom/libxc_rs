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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta832<F: Float>(t12917: F, t17396: F, t1260: F, t17289: F, t13032: F, t17524: F, t17544: F, t3708: F, t13051: F, t56730: F, t12915: F, t16771: F, t247: F, t5384: F, t17763: F, t3636: F, t13085: F, t5391: F, t3568: F, t606: F, t12881: F, t5381: F, t127: F, t12866: F, t17650: F, t5296: F, t17861: F, t3624: F, t1042: F, t1261: F, t12646: F, t12920: F, t13055: F, t13076: F, t16714: F, t1715: F, t17202: F, t17429: F, t17729: F, t17736: F, t17786: F, t3606: F, t3626: F, t3631: F, t3714: F, t44225: F, t44421: F, t5051: F, t5293: F, t53450: F, t5386: F, t12784: F, t17451: F, t17416: F, t3647: F, t11262: F, t1247: F, t5286: F, t17501: F, t3172: F, t3711: F, t13099: F, t43776: F, t12956: F, t17217: F, t12909: F, t17395: F, t12277: F, t12777: F, t12781: F, t12822: F, t12828: F, t12836: F, t12842: F, t12847: F, t12912: F, t17235: F, t17237: F, t17448: F, t21049: F, t21306: F, t44343: F, t44346: F, t5277: F, t53474: F, t17384: F, t12772: F, t17668: F, t3625: F, t17673: F, t12910: F, t12916: F, t17460: F, t17213: F, t13069: F, t5265: F, t17332: F, t17747: F, t17749: F, t11231: F, t1266: F, t12787: F, t17261: F, t17265: F, t20921: F, t44403: F, t44405: F, t44409: F, t44411: F, t44415: F, t44418: F, t45710: F, t5330: F, t5343: F, t12732: F, t1774: F, t1222: F, t16725: F, t17471: F, t16729: F, t13017: F, t5373: F, t44546: F, t5331: F, t5334: F, t17654: F, t17657: F, t56756: F, t17528: F, t44545: F, t5230: F, t12984: F, t5327: F, t12995: F, t17438: F, t1214: F, t1235: F, t1250: F, t12629: F, t17280: F, t17475: F, t17484: F, t17534: F, t17649: F, t1808: F, t20945: F, t2251: F, t3362: F, t3613: F, t3667: F, t371: F, t3718: F, t372: F, t3720: F, t44427: F, t44568: F, t44609: F, t44664: F, t471: F, t482: F, t5351: F, t5405: F, t56179: F, t56376: F, t17303: F, t12886: F, t12627: F, t489: F, t17728: F, t13011: F, t5368: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57049, t57053, t57056, t57063, t57065, t57070) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3105::<F>(t12917, t17396, t1260, t17289, t13032, t17524, t17544, t3708, t13051, t56730, t12915, t16771, t247, t5384);
        let (t57075, t57077, t57083, t57094, t57098) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3106::<F>(t17763, t3636, t13085, t5391, t3568, t606, t12881, t5381, t127, t12866, t17650, t5296);
        let t57103 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3107::<F>(t17861, t3624, t1042, t1260, t1261, t12646, t12920, t13055, t13076, t16714, t1715, t17202, t17429, t17729, t17736, t17786, t3606, t3626, t3631, t3714, t44225, t44421, t5051, t5293, t53450, t5386, t57049, t57053, t57056, t57063, t57065, t57070, t57075, t57077, t57083, t57094, t57098);
        let (t57114, t57118, t57126, t57128, t57136) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3108::<F>(t12784, t17451, t17416, t3647, t11262, t1247, t5286, t17501, t3172, t3711, t13099, t43776);
        let t57150 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3109::<F>(t12956, t17217, t12909, t17395, t1042, t12277, t1261, t12777, t12781, t12822, t12828, t12836, t12842, t12847, t12912, t17235, t17237, t17448, t21049, t21306, t3647, t3711, t44343, t44346, t5277, t53450, t53474, t5381, t57114, t57118, t57126, t57128, t57136);
        let (t57164, t57167, t57170, t57173, t57176) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3110::<F>(t12784, t17384, t12772, t17668, t3625, t17673, t12910, t12916, t17460, t17213, t3172, t5384);
        let t57193 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3111::<F>(t13069, t5265, t1260, t17332, t12916, t17747, t17749, t11231, t1266, t12787, t12828, t17261, t17265, t17729, t20921, t44403, t44405, t44409, t44411, t44415, t44418, t45710, t5330, t5343, t5391, t57164, t57167, t57170, t57173, t57176);
        let (t57200, t57209, t57212, t57214, t57223, t57227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3112::<F>(t12732, t1774, t1222, t16725, t17471, t16729, t13017, t5373, t44546, t5331, t5334, t17654, t17657, t56756);
        let t57254 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3113::<F>(t13032, t17528, t247, t44545, t5230, t5384, t12984, t5327, t12995, t17438, t1214, t1222, t1235, t1250, t12629, t12866, t17280, t17475, t17484, t17534, t17649, t1808, t20945, t2251, t3362, t3613, t3667, t371, t3718, t372, t3720, t44427, t44568, t44609, t44664, t471, t482, t5351, t5405, t56179, t56376, t57200, t57209, t57212, t57214, t57223, t57227);
        let (t57257, t57258, t57264, t57265, t57271, t57273) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3114::<F>(t17303, t3667, t12886, t5381, t12627, t489, t17728, t13011, t5373, t1222, t5368, t697);
    (t57083, t57103, t57150, t57193, t57200, t57254, t57257, t57258, t57264, t57265, t57271, t57273)
}
