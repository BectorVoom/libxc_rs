//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta955 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3182;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3183;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3184;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3185;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3186;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3187;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3188;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3189;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta955<F: Float>(t5245: F, t6587: F, t20820: F, t5265: F, t20851: F, t5362: F, t1042: F, t1121: F, t1261: F, t17505: F, t17654: F, t1794: F, t1808: F, t20907: F, t20914: F, t21272: F, t247: F, t3604: F, t3719: F, t5384: F, t5391: F, t5397: F, t58983: F, t606: F, t69723: F, t70756: F, t70758: F, t71300: F, t78785: F, t21101: F, t5273: F, t1032: F, t1246: F, t24698: F, t1252: F, t12910: F, t20903: F, t24713: F, t3720: F, t5293: F, t5299: F, t5405: F, t56803: F, t56806: F, t6631: F, t6635: F, t69906: F, t70806: F, t70809: F, t70811: F, t70857: F, t12832: F, t17351: F, t17401: F, t20929: F, t20978: F, t21042: F, t24706: F, t3611: F, t44607: F, t5047: F, t5052: F, t56879: F, t57615: F, t57636: F, t57663: F, t57687: F, t57707: F, t6638: F, t69839: F, t70794: F, t83125: F, t5284: F, t6573: F, t1248: F, t1250: F, t12787: F, t12839: F, t17429: F, t17605: F, t17729: F, t20297: F, t20795: F, t20957: F, t21014: F, t21219: F, t21223: F, t21300: F, t24736: F, t3626: F, t4186: F, t44535: F, t5297: F, t5331: F, t5333: F, t5340: F, t57265: F, t58920: F, t59001: F, t82481: F, t82859: F, t82886: F, t1222: F, t140: F, t24830: F, t12840: F, t17747: F, t21020: F, t21134: F, t24573: F, t5308: F, t5312: F, t5373: F, t59355: F, t6611: F, t70914: F, t70942: F, t81177: F, t81198: F, t81202: F, t82293: F, t17471: F, t24236: F, t24679: F, t369: F, t467: F, t475: F, t5390: F, t6601: F, t21177: F, t1235: F, t127: F, t24634: F, t371: F, t1266: F, t17290: F, t21085: F, t21137: F, t21140: F, t21213: F, t5313: F, t5327: F, t57727: F, t6647: F, t20842: F, t17396: F, t20926: F, t12866: F, t58895: F, t6639: F, t1715: F, t17353: F, t17661: F, t17693: F, t1791: F, t20937: F, t21021: F, t21063: F, t21172: F, t44517: F, t5320: F, t58777: F, t59242: F, t70221: F, t70982: F, t17448: F, t21090: F, t471: F, t12916: F, t24730: F, t12784: F, t17753: F, t20800: F, t20836: F, t20941: F, t24787: F, t3625: F, t44521: F, t5401: F, t57660: F, t59196: F, t6421: F, t69832: F, t70890: F, t71009: F, t71020: F, t82838: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t83567, t83592) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3182::<F>(t5245, t6587, t20820, t5265, t20851, t5362, t1042, t1121, t1261, t17505, t17654, t1794, t1808, t20907, t20914, t21272, t247, t3604, t3719, t5384, t5391, t5397, t58983, t606, t69723, t70756, t70758, t71300, t78785);
        let t83617 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3183::<F>(t21101, t5273, t1032, t1246, t24698, t1252, t12910, t20903, t24713, t3720, t5293, t5299, t5405, t56803, t56806, t6631, t6635, t69906, t70806, t70809, t70811, t70857);
        let t83640 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3184::<F>(t12832, t17351, t17401, t17654, t20929, t20978, t21042, t24706, t3604, t3611, t44607, t5047, t5052, t56879, t57615, t57636, t57663, t57687, t57707, t6638, t69839, t70794, t83125);
        let (t83662, t83683) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3185::<F>(t5284, t6573, t1121, t1248, t1250, t12787, t12839, t12910, t17401, t17429, t17605, t17729, t20297, t20795, t20957, t21014, t21219, t21223, t21300, t24736, t3626, t3720, t4186, t44535, t5297, t5331, t5333, t5340, t57265, t58920, t59001, t82481, t82859, t82886);
        let t83712 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3186::<F>(t1222, t140, t24830, t12840, t17429, t17747, t20795, t21020, t21134, t24573, t3626, t5308, t5312, t5331, t5373, t59355, t6611, t70914, t70942, t81177, t81198, t81202, t82293);
        let (t83719, t83725, t83728, t83731, t83735) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3187::<F>(t1222, t17471, t24236, t24679, t369, t467, t475, t5390, t6601, t21177, t5362, t1235, t127, t24634, t371);
        let t83741 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3188::<F>(t1266, t17290, t21085, t21137, t21140, t21213, t5313, t5327, t5373, t57727, t6647, t83719, t83725, t83728, t83731, t83735);
        let (t83760, t83771) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3189::<F>(t20842, t5327, t17396, t20926, t12866, t58895, t6639, t1715, t5284, t17353, t17654, t17661, t17693, t1791, t20937, t21021, t21063, t21172, t3604, t44517, t5320, t58777, t59242, t6611, t70221, t70982, t71300);
        let (t83792, t83808) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3190::<F>(t17448, t21090, t1248, t1794, t471, t12916, t24730, t5340, t12784, t12787, t17753, t20800, t20836, t20941, t24787, t3625, t3720, t44521, t5331, t5333, t5401, t57660, t59196, t6421, t69832, t70890, t71009, t71020, t82838, t82886);
    (t83567, t83592, t83617, t83640, t83662, t83683, t83712, t83741, t83760, t83771, t83792, t83808)
}
