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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3155;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3156;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3157;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3158;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3159;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3160;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3161;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta952(t5245: f64, t6622: f64, t1250: f64, t16714: f64, t17396: f64, t17736: f64, t17747: f64, t19680: f64, t20802: f64, t20950: f64, t20956: f64, t21014: f64, t21259: f64, t21310: f64, t3626: f64, t3718: f64, t3720: f64, t44225: f64, t44609: f64, t5230: f64, t5341: f64, t5352: f64, t57005: f64, t57040: f64, t59011: f64, t6429: f64, t6690: f64, t72002: f64, t82725: f64, t82881: f64, t82886: f64, t1042: f64, t1261: f64, t17550: f64, t17569: f64, t20876: f64, t20880: f64, t24546: f64, t24668: f64, t44202: f64, t44526: f64, t5299: f64, t56246: f64, t57053: f64, t6619: f64, t69899: f64, t69910: f64, t69916: f64, t69968: f64, t78785: f64, t78790: f64, t20783: f64, t12866: f64, t17693: f64, t17694: f64, t20820: f64, t5268: f64, t5287: f64, t69936: f64, t69939: f64, t69947: f64, t69961: f64, t69964: f64, t69966: f64, t78770: f64, t82587: f64, t82591: f64, t12787: f64, t20792: f64, t20800: f64, t20811: f64, t21143: f64, t3362: f64, t4181: f64, t5302: f64, t5304: f64, t5340: f64, t5381: f64, t57056: f64, t6573: f64, t6631: f64, t69971: f64, t69984: f64, t70006: f64, t70008: f64, t1222: f64, t140: f64, t24816: f64, t24820: f64, t12915: f64, t247: f64, t24713: f64, t5384: f64, t13046: f64, t13053: f64, t17307: f64, t17654: f64, t20765: f64, t20932: f64, t20933: f64, t20941: f64, t3604: f64, t3611: f64, t44517: f64, t5052: f64, t5386: f64, t5390: f64, t5401: f64, t56997: f64, t57663: f64, t59066: f64, t69839: f64, t71112: f64, t71300: f64, t21272: f64, t5378: f64, t44799: f64, t82578: f64, t1794: f64, t5825: f64, t1469: f64, t4186: f64, t12772: f64, t24793: f64, t3625: f64, t17661: f64, t17799: f64, t20934: f64, t20947: f64, t21173: f64, t21218: f64, t21306: f64, t57660: f64, t58899: f64, t59362: f64, t70032: f64, t70496: f64, t24803: f64, t44425: f64, t17448: f64, t17605: f64, t17729: f64, t20265: f64, t21020: f64, t21040: f64, t21157: f64, t21161: f64, t24240: f64, t5402: f64, t5405: f64, t6638: f64, t70039: f64, t70044: f64, t70819: f64, t70944: f64, t82481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82899, t82904) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3155(t5245, t6622, t1250, t16714, t17396, t17736, t17747, t19680, t20802, t20950, t20956, t21014, t21259, t21310, t3626, t3718, t3720, t44225, t44609, t5230, t5341, t5352, t57005, t57040, t59011, t6429, t6690, t72002, t82725, t82881, t82886);
        let t82929 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3156(t1042, t1261, t17550, t17569, t20876, t20880, t24546, t24668, t44202, t44526, t5299, t56246, t57053, t6619, t69899, t69910, t69916, t69968, t78785, t78790);
        let t82950 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3157(t17569, t20783, t1042, t1261, t12866, t17693, t17694, t20820, t5268, t5287, t69936, t69939, t69947, t69961, t69964, t69966, t78770, t82587, t82591);
        let t82978 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3158(t1042, t1261, t12787, t17569, t17736, t20792, t20800, t20811, t20950, t21143, t3362, t3720, t4181, t5302, t5304, t5340, t5381, t57056, t6573, t6631, t69971, t69984, t70006, t70008, t78770);
        let t83016 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3159(t1222, t140, t24816, t24820, t12915, t247, t24713, t5384, t12866, t13046, t13053, t17307, t17654, t20765, t20932, t20933, t20941, t3604, t3611, t44517, t5052, t5386, t5390, t5401, t56997, t57663, t59066, t69839, t71112, t71300);
        let (t83018, t83024, t83033, t83034, t83040, t83047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3160(t21272, t5378, t44799, t82578, t1794, t5825, t1250, t1469, t4186, t12772, t24793, t3625);
        let t83051 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3161(t12866, t17661, t17693, t17694, t17799, t20934, t20947, t21173, t21218, t21306, t57660, t58899, t59362, t70032, t70496, t83018, t83024, t83034, t83040, t83047);
        let t83081 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3162(t24803, t3625, t44425, t12787, t17448, t17605, t17729, t20265, t21020, t21040, t21157, t21161, t24240, t3626, t5402, t5405, t6638, t70039, t70044, t70819, t70944, t82481);
    (t82899, t82904, t82929, t82950, t82978, t83016, t83024, t83033, t83034, t83040, t83051, t83081)
}
