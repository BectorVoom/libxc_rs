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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta955(t5245: f64, t6587: f64, t20820: f64, t5265: f64, t20851: f64, t5362: f64, t1042: f64, t1121: f64, t1261: f64, t17505: f64, t17654: f64, t1794: f64, t1808: f64, t20907: f64, t20914: f64, t21272: f64, t247: f64, t3604: f64, t3719: f64, t5384: f64, t5391: f64, t5397: f64, t58983: f64, t606: f64, t69723: f64, t70756: f64, t70758: f64, t71300: f64, t78785: f64, t21101: f64, t5273: f64, t1032: f64, t1246: f64, t24698: f64, t1252: f64, t12910: f64, t20903: f64, t24713: f64, t3720: f64, t5293: f64, t5299: f64, t5405: f64, t56803: f64, t56806: f64, t6631: f64, t6635: f64, t69906: f64, t70806: f64, t70809: f64, t70811: f64, t70857: f64, t12832: f64, t17351: f64, t17401: f64, t20929: f64, t20978: f64, t21042: f64, t24706: f64, t3611: f64, t44607: f64, t5047: f64, t5052: f64, t56879: f64, t57615: f64, t57636: f64, t57663: f64, t57687: f64, t57707: f64, t6638: f64, t69839: f64, t70794: f64, t83125: f64, t5284: f64, t6573: f64, t1248: f64, t1250: f64, t12787: f64, t12839: f64, t17429: f64, t17605: f64, t17729: f64, t20297: f64, t20795: f64, t20957: f64, t21014: f64, t21219: f64, t21223: f64, t21300: f64, t24736: f64, t3626: f64, t4186: f64, t44535: f64, t5297: f64, t5331: f64, t5333: f64, t5340: f64, t57265: f64, t58920: f64, t59001: f64, t82481: f64, t82859: f64, t82886: f64, t1222: f64, t140: f64, t24830: f64, t12840: f64, t17747: f64, t21020: f64, t21134: f64, t24573: f64, t5308: f64, t5312: f64, t5373: f64, t59355: f64, t6611: f64, t70914: f64, t70942: f64, t81177: f64, t81198: f64, t81202: f64, t82293: f64, t17471: f64, t24236: f64, t24679: f64, t369: f64, t467: f64, t475: f64, t5390: f64, t6601: f64, t21177: f64, t1235: f64, t127: f64, t24634: f64, t371: f64, t1266: f64, t17290: f64, t21085: f64, t21137: f64, t21140: f64, t21213: f64, t5313: f64, t5327: f64, t57727: f64, t6647: f64, t20842: f64, t17396: f64, t20926: f64, t12866: f64, t58895: f64, t6639: f64, t1715: f64, t17353: f64, t17661: f64, t17693: f64, t1791: f64, t20937: f64, t21021: f64, t21063: f64, t21172: f64, t44517: f64, t5320: f64, t58777: f64, t59242: f64, t70221: f64, t70982: f64, t17448: f64, t21090: f64, t471: f64, t12916: f64, t24730: f64, t12784: f64, t17753: f64, t20800: f64, t20836: f64, t20941: f64, t24787: f64, t3625: f64, t44521: f64, t5401: f64, t57660: f64, t59196: f64, t6421: f64, t69832: f64, t70890: f64, t71009: f64, t71020: f64, t82838: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83567, t83592) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3182(t5245, t6587, t20820, t5265, t20851, t5362, t1042, t1121, t1261, t17505, t17654, t1794, t1808, t20907, t20914, t21272, t247, t3604, t3719, t5384, t5391, t5397, t58983, t606, t69723, t70756, t70758, t71300, t78785);
        let t83617 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3183(t21101, t5273, t1032, t1246, t24698, t1252, t12910, t20903, t24713, t3720, t5293, t5299, t5405, t56803, t56806, t6631, t6635, t69906, t70806, t70809, t70811, t70857);
        let t83640 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3184(t12832, t17351, t17401, t17654, t20929, t20978, t21042, t24706, t3604, t3611, t44607, t5047, t5052, t56879, t57615, t57636, t57663, t57687, t57707, t6638, t69839, t70794, t83125);
        let (t83662, t83683) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3185(t5284, t6573, t1121, t1248, t1250, t12787, t12839, t12910, t17401, t17429, t17605, t17729, t20297, t20795, t20957, t21014, t21219, t21223, t21300, t24736, t3626, t3720, t4186, t44535, t5297, t5331, t5333, t5340, t57265, t58920, t59001, t82481, t82859, t82886);
        let t83712 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3186(t1222, t140, t24830, t12840, t17429, t17747, t20795, t21020, t21134, t24573, t3626, t5308, t5312, t5331, t5373, t59355, t6611, t70914, t70942, t81177, t81198, t81202, t82293);
        let (t83719, t83725, t83728, t83731, t83735) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3187(t1222, t17471, t24236, t24679, t369, t467, t475, t5390, t6601, t21177, t5362, t1235, t127, t24634, t371);
        let t83741 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3188(t1266, t17290, t21085, t21137, t21140, t21213, t5313, t5327, t5373, t57727, t6647, t83719, t83725, t83728, t83731, t83735);
        let (t83760, t83771) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3189(t20842, t5327, t17396, t20926, t12866, t58895, t6639, t1715, t5284, t17353, t17654, t17661, t17693, t1791, t20937, t21021, t21063, t21172, t3604, t44517, t5320, t58777, t59242, t6611, t70221, t70982, t71300);
        let (t83792, t83808) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3190(t17448, t21090, t1248, t1794, t471, t12916, t24730, t5340, t12784, t12787, t17753, t20800, t20836, t20941, t24787, t3625, t3720, t44521, t5331, t5333, t5401, t57660, t59196, t6421, t69832, t70890, t71009, t71020, t82838, t82886);
    (t83567, t83592, t83617, t83640, t83662, t83683, t83712, t83741, t83760, t83771, t83792, t83808)
}
