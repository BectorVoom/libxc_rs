//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta952 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3155;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3156;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3157;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3158;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3159;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3160;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3161;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta952<F: Float>(t5245: F, t6622: F, t1250: F, t16714: F, t17396: F, t17736: F, t17747: F, t19680: F, t20802: F, t20950: F, t20956: F, t21014: F, t21259: F, t21310: F, t3626: F, t3718: F, t3720: F, t44225: F, t44609: F, t5230: F, t5341: F, t5352: F, t57005: F, t57040: F, t59011: F, t6429: F, t6690: F, t72002: F, t82725: F, t82881: F, t82886: F, t1042: F, t1261: F, t17550: F, t17569: F, t20876: F, t20880: F, t24546: F, t24668: F, t44202: F, t44526: F, t5299: F, t56246: F, t57053: F, t6619: F, t69899: F, t69910: F, t69916: F, t69968: F, t78785: F, t78790: F, t20783: F, t12866: F, t17693: F, t17694: F, t20820: F, t5268: F, t5287: F, t69936: F, t69939: F, t69947: F, t69961: F, t69964: F, t69966: F, t78770: F, t82587: F, t82591: F, t12787: F, t20792: F, t20800: F, t20811: F, t21143: F, t3362: F, t4181: F, t5302: F, t5304: F, t5340: F, t5381: F, t57056: F, t6573: F, t6631: F, t69971: F, t69984: F, t70006: F, t70008: F, t1222: F, t140: F, t24816: F, t24820: F, t12915: F, t247: F, t24713: F, t5384: F, t13046: F, t13053: F, t17307: F, t17654: F, t20765: F, t20932: F, t20933: F, t20941: F, t3604: F, t3611: F, t44517: F, t5052: F, t5386: F, t5390: F, t5401: F, t56997: F, t57663: F, t59066: F, t69839: F, t71112: F, t71300: F, t21272: F, t5378: F, t44799: F, t82578: F, t1794: F, t5825: F, t1469: F, t4186: F, t12772: F, t24793: F, t3625: F, t17661: F, t17799: F, t20934: F, t20947: F, t21173: F, t21218: F, t21306: F, t57660: F, t58899: F, t59362: F, t70032: F, t70496: F, t24803: F, t44425: F, t17448: F, t17605: F, t17729: F, t20265: F, t21020: F, t21040: F, t21157: F, t21161: F, t24240: F, t5402: F, t5405: F, t6638: F, t70039: F, t70044: F, t70819: F, t70944: F, t82481: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t82899, t82904) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3155::<F>(t5245, t6622, t1250, t16714, t17396, t17736, t17747, t19680, t20802, t20950, t20956, t21014, t21259, t21310, t3626, t3718, t3720, t44225, t44609, t5230, t5341, t5352, t57005, t57040, t59011, t6429, t6690, t72002, t82725, t82881, t82886);
        let t82929 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3156::<F>(t1042, t1261, t17550, t17569, t20876, t20880, t24546, t24668, t44202, t44526, t5299, t56246, t57053, t6619, t69899, t69910, t69916, t69968, t78785, t78790);
        let t82950 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3157::<F>(t17569, t20783, t1042, t1261, t12866, t17693, t17694, t20820, t5268, t5287, t69936, t69939, t69947, t69961, t69964, t69966, t78770, t82587, t82591);
        let t82978 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3158::<F>(t1042, t1261, t12787, t17569, t17736, t20792, t20800, t20811, t20950, t21143, t3362, t3720, t4181, t5302, t5304, t5340, t5381, t57056, t6573, t6631, t69971, t69984, t70006, t70008, t78770);
        let t83016 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3159::<F>(t1222, t140, t24816, t24820, t12915, t247, t24713, t5384, t12866, t13046, t13053, t17307, t17654, t20765, t20932, t20933, t20941, t3604, t3611, t44517, t5052, t5386, t5390, t5401, t56997, t57663, t59066, t69839, t71112, t71300);
        let (t83018, t83024, t83033, t83034, t83040, t83047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3160::<F>(t21272, t5378, t44799, t82578, t1794, t5825, t1250, t1469, t4186, t12772, t24793, t3625);
        let t83051 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3161::<F>(t12866, t17661, t17693, t17694, t17799, t20934, t20947, t21173, t21218, t21306, t57660, t58899, t59362, t70032, t70496, t83018, t83024, t83034, t83040, t83047);
        let t83081 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3162::<F>(t24803, t3625, t44425, t12787, t17448, t17605, t17729, t20265, t21020, t21040, t21157, t21161, t24240, t3626, t5402, t5405, t6638, t70039, t70044, t70819, t70944, t82481);
    (t82899, t82904, t82929, t82950, t82978, t83016, t83024, t83033, t83034, t83040, t83051, t83081)
}
