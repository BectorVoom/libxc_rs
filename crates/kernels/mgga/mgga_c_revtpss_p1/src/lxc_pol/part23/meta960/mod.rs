//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta960 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3233;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3234;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3235;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3236;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3237;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3238;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3239;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3240;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3241;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3242;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3243;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta960<F: Float>(t22648: F, t602: F, t13368: F, t13371: F, t13378: F, t13381: F, t18281: F, t19680: F, t21784: F, t21794: F, t22671: F, t22688: F, t2299: F, t2306: F, t4186: F, t4227: F, t4232: F, t46001: F, t46014: F, t5825: F, t606: F, t633: F, t637: F, t76397: F, t1471: F, t1487: F, t1494: F, t21769: F, t21805: F, t22718: F, t22739: F, t4188: F, t4191: F, t4217: F, t4218: F, t4238: F, t5819: F, t5855: F, t5869: F, t607: F, t628: F, t71: F, t77: F, t85: F, t1469: F, t1486: F, t72: F, t1927: F, t21686: F, t21687: F, t21727: F, t22662: F, t22672: F, t36: F, t4196: F, t608: F, t60823: F, t627: F, t6977: F, t70: F, t7719: F, t78770: F, t21695: F, t21698: F, t21699: F, t21702: F, t22673: F, t22676: F, t4181: F, t4187: F, t5826: F, t5827: F, t5854: F, t641: F, t1480: F, t21754: F, t21762: F, t21765: F, t22689: F, t22695: F, t22700: F, t4214: F, t44: F, t46090: F, t48: F, t56: F, t5843: F, t60: F, t60308: F, t60311: F, t614: F, t620: F, t77513: F, t13302: F, t13324: F, t21732: F, t21755: F, t21758: F, t22692: F, t2275: F, t2282: F, t4201: F, t4210: F, t4211: F, t46065: F, t46074: F, t1470: F, t21690: F, t21707: F, t21710: F, t21713: F, t21768: F, t22665: F, t22681: F, t22719: F, t38: F, t4182: F, t5820: F, t5830: F, t10301: F, t10309: F, t1497: F, t21809: F, t2242: F, t2247: F, t22656: F, t22659: F, t22742: F, t4173: F, t4241: F, t45963: F, t45972: F, t5816: F, t5872: F, t603: F, t644: F, t5: F, t13269: F, t13272: F, t21663: F, t21674: F, t21677: F, t21682: F, t4178: F, t45931: F, t45933: F, t45941: F, t45944: F, t45952: F, t60214: F, t60215: F, t60216: F, t60217: F, t60218: F, t60221: F, t60224: F, t60670: F, t60673: F, t91: F, t117: F, t118: F, t13426: F, t18227: F, t18232: F, t18235: F, t18242: F, t18245: F, t1843: F, t21814: F, t25043: F, t4248: F, t4297: F, t508: F, t5921: F, t649: F, t651: F, t670: F, t671: F, t75931: F, t75941: F, t81110: F, t85032: F, t4245: F, t5883: F, t1310: F, t1502: F, t18220: F, t1911: F, t21658: F, t21881: F, t21882: F, t22525: F, t22639: F, t22747: F, t27123: F, t30138: F, t4246: F, t4292: F, t4293: F, t5517: F, t5877: F, t5884: F, t6765: F, t5876: F, t1519: F, t21891: F, t22578: F, t2322: F, t27126: F, t4254: F, t4257: F, t5887: F, t5920: F, t75439: F, t7732: F) -> (F, F, F, F, F, F) {
        let (t85037, t85125) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3233::<F>(t22648, t602, t13368, t13371, t13378, t13381, t18281, t19680, t21784, t21794, t22671, t22688, t2299, t2306, t4186, t4227, t4232, t46001, t46014, t5825, t606, t633, t637, t76397);
        let t85141 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3234::<F>(t1471, t1487, t1494, t21769, t21805, t22718, t22739, t4188, t4191, t4217, t4218, t4238, t5819, t5855, t5869, t607, t628, t71, t77, t85, t85125);
        let t85177 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3235::<F>(t1469, t1486, t72, t1494, t18281, t1927, t21686, t21687, t21727, t22662, t22672, t22739, t36, t4186, t4196, t5825, t5869, t608, t60823, t627, t6977, t70, t76397, t7719, t78770, t85);
        let t85206 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3236::<F>(t1486, t1494, t19680, t21695, t21698, t21699, t21702, t22673, t22676, t4181, t4187, t4217, t4238, t5826, t5827, t5854, t641, t85);
        let t85255 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3237::<F>(t1480, t21754, t21762, t21765, t22689, t22695, t22700, t4186, t4214, t44, t46090, t48, t56, t5843, t60, t60308, t60311, t614, t620, t76397, t77513);
        let t85295 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3238::<F>(t13302, t13324, t1480, t18281, t21732, t21755, t21758, t22671, t22688, t22692, t2275, t2282, t4186, t4201, t4210, t4211, t44, t46065, t46074, t56, t5825, t5843, t606, t614);
        let t85300 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3239::<F>(t1470, t1494, t21690, t21707, t21710, t21713, t21768, t22665, t22681, t22719, t38, t4182, t4238, t5820, t5830, t5869, t641, t85, t85255, t85295);
        let t85305 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3240::<F>(t10301, t10309, t1497, t21809, t2242, t2247, t22656, t22659, t22742, t4173, t4241, t45963, t45972, t5816, t5872, t603, t644, t85141, t85177, t85206, t85300);
        let t85307 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3241::<F>(t5, t13269, t13272, t1497, t21663, t21674, t21677, t21682, t4178, t4241, t45931, t45933, t45941, t45944, t45952, t5816, t5872, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60670, t60673, t644, t85037, t85305, t91);
        let (t85308, t85312) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3242::<F>(t117, t85307, t118, t13426, t18227, t18232, t18235, t18242, t18245, t1843, t21814, t25043, t4248, t4297, t508, t5921, t649, t651, t670, t671, t75931, t75941, t81110, t85032);
        let (t85329, t85343) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3243::<F>(t4245, t5883, t1310, t1502, t18220, t1843, t1911, t21658, t21881, t21882, t22525, t22639, t22747, t27123, t30138, t4246, t4248, t4292, t4293, t508, t5517, t5877, t5884, t5921, t651, t6765);
        let (t85360, t85373) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3244::<F>(t5876, t670, t13426, t1519, t18227, t18242, t18245, t21882, t21891, t22578, t2322, t27126, t4248, t4254, t4257, t4293, t5517, t5887, t5920, t5921, t651, t75439, t7732);
    (t85308, t85312, t85329, t85343, t85360, t85373)
}
