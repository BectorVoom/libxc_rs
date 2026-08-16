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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta960(t22648: f64, t602: f64, t13368: f64, t13371: f64, t13378: f64, t13381: f64, t18281: f64, t19680: f64, t21784: f64, t21794: f64, t22671: f64, t22688: f64, t2299: f64, t2306: f64, t4186: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t5825: f64, t606: f64, t633: f64, t637: f64, t76397: f64, t1471: f64, t1487: f64, t1494: f64, t21769: f64, t21805: f64, t22718: f64, t22739: f64, t4188: f64, t4191: f64, t4217: f64, t4218: f64, t4238: f64, t5819: f64, t5855: f64, t5869: f64, t607: f64, t628: f64, t71: f64, t77: f64, t85: f64, t1469: f64, t1486: f64, t72: f64, t1927: f64, t21686: f64, t21687: f64, t21727: f64, t22662: f64, t22672: f64, t36: f64, t4196: f64, t608: f64, t60823: f64, t627: f64, t6977: f64, t70: f64, t7719: f64, t78770: f64, t21695: f64, t21698: f64, t21699: f64, t21702: f64, t22673: f64, t22676: f64, t4181: f64, t4187: f64, t5826: f64, t5827: f64, t5854: f64, t641: f64, t1480: f64, t21754: f64, t21762: f64, t21765: f64, t22689: f64, t22695: f64, t22700: f64, t4214: f64, t44: f64, t46090: f64, t48: f64, t56: f64, t5843: f64, t60: f64, t60308: f64, t60311: f64, t614: f64, t620: f64, t77513: f64, t13302: f64, t13324: f64, t21732: f64, t21755: f64, t21758: f64, t22692: f64, t2275: f64, t2282: f64, t4201: f64, t4210: f64, t4211: f64, t46065: f64, t46074: f64, t1470: f64, t21690: f64, t21707: f64, t21710: f64, t21713: f64, t21768: f64, t22665: f64, t22681: f64, t22719: f64, t38: f64, t4182: f64, t5820: f64, t5830: f64, t10301: f64, t10309: f64, t1497: f64, t21809: f64, t2242: f64, t2247: f64, t22656: f64, t22659: f64, t22742: f64, t4173: f64, t4241: f64, t45963: f64, t45972: f64, t5816: f64, t5872: f64, t603: f64, t644: f64, t5: f64, t13269: f64, t13272: f64, t21663: f64, t21674: f64, t21677: f64, t21682: f64, t4178: f64, t45931: f64, t45933: f64, t45941: f64, t45944: f64, t45952: f64, t60214: f64, t60215: f64, t60216: f64, t60217: f64, t60218: f64, t60221: f64, t60224: f64, t60670: f64, t60673: f64, t91: f64, t117: f64, t118: f64, t13426: f64, t18227: f64, t18232: f64, t18235: f64, t18242: f64, t18245: f64, t1843: f64, t21814: f64, t25043: f64, t4248: f64, t4297: f64, t508: f64, t5921: f64, t649: f64, t651: f64, t670: f64, t671: f64, t75931: f64, t75941: f64, t81110: f64, t85032: f64, t4245: f64, t5883: f64, t1310: f64, t1502: f64, t18220: f64, t1911: f64, t21658: f64, t21881: f64, t21882: f64, t22525: f64, t22639: f64, t22747: f64, t27123: f64, t30138: f64, t4246: f64, t4292: f64, t4293: f64, t5517: f64, t5877: f64, t5884: f64, t6765: f64, t5876: f64, t1519: f64, t21891: f64, t22578: f64, t2322: f64, t27126: f64, t4254: f64, t4257: f64, t5887: f64, t5920: f64, t75439: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t85037, t85125) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3233(t22648, t602, t13368, t13371, t13378, t13381, t18281, t19680, t21784, t21794, t22671, t22688, t2299, t2306, t4186, t4227, t4232, t46001, t46014, t5825, t606, t633, t637, t76397);
        let t85141 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3234(t1471, t1487, t1494, t21769, t21805, t22718, t22739, t4188, t4191, t4217, t4218, t4238, t5819, t5855, t5869, t607, t628, t71, t77, t85, t85125);
        let t85177 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3235(t1469, t1486, t72, t1494, t18281, t1927, t21686, t21687, t21727, t22662, t22672, t22739, t36, t4186, t4196, t5825, t5869, t608, t60823, t627, t6977, t70, t76397, t7719, t78770, t85);
        let t85206 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3236(t1486, t1494, t19680, t21695, t21698, t21699, t21702, t22673, t22676, t4181, t4187, t4217, t4238, t5826, t5827, t5854, t641, t85);
        let t85255 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3237(t1480, t21754, t21762, t21765, t22689, t22695, t22700, t4186, t4214, t44, t46090, t48, t56, t5843, t60, t60308, t60311, t614, t620, t76397, t77513);
        let t85295 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3238(t13302, t13324, t1480, t18281, t21732, t21755, t21758, t22671, t22688, t22692, t2275, t2282, t4186, t4201, t4210, t4211, t44, t46065, t46074, t56, t5825, t5843, t606, t614);
        let t85300 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3239(t1470, t1494, t21690, t21707, t21710, t21713, t21768, t22665, t22681, t22719, t38, t4182, t4238, t5820, t5830, t5869, t641, t85, t85255, t85295);
        let t85305 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3240(t10301, t10309, t1497, t21809, t2242, t2247, t22656, t22659, t22742, t4173, t4241, t45963, t45972, t5816, t5872, t603, t644, t85141, t85177, t85206, t85300);
        let t85307 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3241(t5, t13269, t13272, t1497, t21663, t21674, t21677, t21682, t4178, t4241, t45931, t45933, t45941, t45944, t45952, t5816, t5872, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60670, t60673, t644, t85037, t85305, t91);
        let (t85308, t85312) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3242(t117, t85307, t118, t13426, t18227, t18232, t18235, t18242, t18245, t1843, t21814, t25043, t4248, t4297, t508, t5921, t649, t651, t670, t671, t75931, t75941, t81110, t85032);
        let (t85329, t85343) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3243(t4245, t5883, t1310, t1502, t18220, t1843, t1911, t21658, t21881, t21882, t22525, t22639, t22747, t27123, t30138, t4246, t4248, t4292, t4293, t508, t5517, t5877, t5884, t5921, t651, t6765);
        let (t85360, t85373) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3244(t5876, t670, t13426, t1519, t18227, t18242, t18245, t21882, t21891, t22578, t2322, t27126, t4248, t4254, t4257, t4293, t5517, t5887, t5920, t5921, t651, t75439, t7732);
    (t85308, t85312, t85329, t85343, t85360, t85373)
}
