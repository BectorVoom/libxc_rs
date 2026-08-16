//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1057 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3747;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3748;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3749;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3750;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3751;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1057<F: Float>(t17395: F, t5436: F, t17435: F, t5323: F, t3670: F, t6594: F, t3718: F, t44546: F, t6689: F, t1222: F, t17240: F, t20318: F, t1263: F, t372: F, t6622: F, t17605: F, t17635: F, t17646: F, t17654: F, t17657: F, t17781: F, t21306: F, t21310: F, t3631: F, t3674: F, t3720: F, t44422: F, t44797: F, t5340: F, t5341: F, t57348: F, t6611: F, t70311: F, t19661: F, t5405: F, t17241: F, t5373: F, t20766: F, t56756: F, t12809: F, t16696: F, t17247: F, t17250: F, t17429: F, t17476: F, t17651: F, t17693: F, t20800: F, t20806: F, t21213: F, t3689: F, t3694: F, t57660: F, t58899: F, t58975: F, t58997: F, t20937: F, t20310: F, t12832: F, t12866: F, t17170: F, t17351: F, t17353: F, t17420: F, t17513: F, t17703: F, t17705: F, t21049: F, t21259: F, t3603: F, t3604: F, t3611: F, t44510: F, t44517: F, t5332: F, t5401: F, t59040: F, t59043: F, t59062: F, t69839: F, t70633: F, t20306: F, t12772: F, t21156: F, t3625: F, t17456: F, t17639: F, t17645: F, t17661: F, t44823: F, t44829: F, t44838: F, t44884: F, t5308: F, t5312: F, t59162: F, t68269: F, t68273: F, t68317: F, t68328: F, t127: F, t20944: F, t20946: F, t1285: F, t57659: F, t17350: F, t17934: F, t5297: F, t606: F, t1248: F, t12787: F, t13046: F, t1715: F, t17380: F, t17455: F, t17658: F, t17662: F, t17687: F, t17688: F, t17696: F, t1790: F, t21040: F, t3588: F, t44550: F, t44951: F, t5056: F, t56997: F, t57663: F, t59078: F, t59362: F, t71200: F, t73: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t71275, t71278, t71280, t71294, t71297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3747::<F>(t17395, t5436, t17435, t5323, t3670, t6594, t3718, t44546, t6689, t1222, t17240, t20318);
        let (t71300, t71304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3748::<F>(t1263, t372, t6622, t17605, t17635, t17646, t17654, t17657, t17781, t21306, t21310, t3631, t3674, t3720, t44422, t44797, t5340, t5341, t57348, t6611, t70311, t71275, t71278, t71280, t71294, t71297);
        let (t71314, t71334) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3749::<F>(t19661, t5405, t17241, t5373, t17654, t20766, t56756, t12809, t16696, t17247, t17250, t17429, t17476, t17651, t17693, t20800, t20806, t21213, t3689, t3694, t3720, t57660, t58899, t58975, t58997);
        let t71375 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3750::<F>(t17693, t20937, t56756, t1222, t17240, t20310, t12832, t12866, t17170, t17351, t17353, t17420, t17513, t17703, t17705, t20800, t21049, t21259, t3603, t3604, t3611, t3720, t44510, t44517, t5332, t5340, t5401, t59040, t59043, t59062, t69839, t70633);
        let t71406 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3751::<F>(t1222, t17240, t20306, t12772, t21156, t3625, t12866, t17456, t17639, t17645, t17661, t44823, t44829, t44838, t44884, t5308, t5312, t59162, t68269, t68273, t68317, t68328);
        let (t71440, t71452, t71457) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3752::<F>(t127, t17693, t20944, t20946, t1285, t57659, t17350, t17934, t5297, t606, t1248, t12787, t12866, t13046, t1715, t17353, t17380, t17455, t17654, t17658, t17662, t17687, t17688, t17696, t1790, t21040, t3588, t3604, t3625, t372, t44550, t44951, t5056, t56997, t57663, t59078, t59362, t71200, t71314, t73);
    (t71300, t71304, t71314, t71334, t71375, t71406, t71440, t71452, t71457)
}
