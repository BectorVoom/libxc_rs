//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1057 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3747;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3748;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3749;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3750;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3751;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1057(t17395: f64, t5436: f64, t17435: f64, t5323: f64, t3670: f64, t6594: f64, t3718: f64, t44546: f64, t6689: f64, t1222: f64, t17240: f64, t20318: f64, t1263: f64, t372: f64, t6622: f64, t17605: f64, t17635: f64, t17646: f64, t17654: f64, t17657: f64, t17781: f64, t21306: f64, t21310: f64, t3631: f64, t3674: f64, t3720: f64, t44422: f64, t44797: f64, t5340: f64, t5341: f64, t57348: f64, t6611: f64, t70311: f64, t19661: f64, t5405: f64, t17241: f64, t5373: f64, t20766: f64, t56756: f64, t12809: f64, t16696: f64, t17247: f64, t17250: f64, t17429: f64, t17476: f64, t17651: f64, t17693: f64, t20800: f64, t20806: f64, t21213: f64, t3689: f64, t3694: f64, t57660: f64, t58899: f64, t58975: f64, t58997: f64, t20937: f64, t20310: f64, t12832: f64, t12866: f64, t17170: f64, t17351: f64, t17353: f64, t17420: f64, t17513: f64, t17703: f64, t17705: f64, t21049: f64, t21259: f64, t3603: f64, t3604: f64, t3611: f64, t44510: f64, t44517: f64, t5332: f64, t5401: f64, t59040: f64, t59043: f64, t59062: f64, t69839: f64, t70633: f64, t20306: f64, t12772: f64, t21156: f64, t3625: f64, t17456: f64, t17639: f64, t17645: f64, t17661: f64, t44823: f64, t44829: f64, t44838: f64, t44884: f64, t5308: f64, t5312: f64, t59162: f64, t68269: f64, t68273: f64, t68317: f64, t68328: f64, t127: f64, t20944: f64, t20946: f64, t1285: f64, t57659: f64, t17350: f64, t17934: f64, t5297: f64, t606: f64, t1248: f64, t12787: f64, t13046: f64, t1715: f64, t17380: f64, t17455: f64, t17658: f64, t17662: f64, t17687: f64, t17688: f64, t17696: f64, t1790: f64, t21040: f64, t3588: f64, t44550: f64, t44951: f64, t5056: f64, t56997: f64, t57663: f64, t59078: f64, t59362: f64, t71200: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71275, t71278, t71280, t71294, t71297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3747(t17395, t5436, t17435, t5323, t3670, t6594, t3718, t44546, t6689, t1222, t17240, t20318);
        let (t71300, t71304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3748(t1263, t372, t6622, t17605, t17635, t17646, t17654, t17657, t17781, t21306, t21310, t3631, t3674, t3720, t44422, t44797, t5340, t5341, t57348, t6611, t70311, t71275, t71278, t71280, t71294, t71297);
        let (t71314, t71334) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3749(t19661, t5405, t17241, t5373, t17654, t20766, t56756, t12809, t16696, t17247, t17250, t17429, t17476, t17651, t17693, t20800, t20806, t21213, t3689, t3694, t3720, t57660, t58899, t58975, t58997);
        let t71375 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3750(t17693, t20937, t56756, t1222, t17240, t20310, t12832, t12866, t17170, t17351, t17353, t17420, t17513, t17703, t17705, t20800, t21049, t21259, t3603, t3604, t3611, t3720, t44510, t44517, t5332, t5340, t5401, t59040, t59043, t59062, t69839, t70633);
        let t71406 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3751(t1222, t17240, t20306, t12772, t21156, t3625, t12866, t17456, t17639, t17645, t17661, t44823, t44829, t44838, t44884, t5308, t5312, t59162, t68269, t68273, t68317, t68328);
        let (t71440, t71452, t71457) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3752(t127, t17693, t20944, t20946, t1285, t57659, t17350, t17934, t5297, t606, t1248, t12787, t12866, t13046, t1715, t17353, t17380, t17455, t17654, t17658, t17662, t17687, t17688, t17696, t1790, t21040, t3588, t3604, t3625, t372, t44550, t44951, t5056, t56997, t57663, t59078, t59362, t71200, t71314, t73);
    (t71300, t71304, t71314, t71334, t71375, t71406, t71440, t71452, t71457)
}
