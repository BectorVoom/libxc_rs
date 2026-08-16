//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta416 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1547;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1548;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1549;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1550;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1551;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1552;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1553;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1554;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1555;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1556;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1557;
use chunk11::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta416(t3057: f64, t4980: f64, t3059: f64, t3151: f64, t11223: f64, t3286: f64, t11200: f64, t11213: f64, t3046: f64, t4995: f64, t1071: f64, t11247: f64, t3143: f64, t42859: f64, t342: f64, t3154: f64, t43351: f64, t1089: f64, t11788: f64, t12066: f64, t12073: f64, t12128: f64, t12150: f64, t12163: f64, t12167: f64, t12168: f64, t3204: f64, t3287: f64, t3288: f64, t3304: f64, t3318: f64, t42610: f64, t43292: f64, t43348: f64, t4976: f64, t989: f64, t3259: f64, t12032: f64, t359: f64, t3043: f64, t3298: f64, t1024: f64, t1082: f64, t1087: f64, t12122: f64, t12132: f64, t12133: f64, t16410: f64, t16520: f64, t3075: f64, t3118: f64, t3133: f64, t3153: f64, t3299: f64, t3305: f64, t378: f64, t380: f64, t42760: f64, t42852: f64, t42909: f64, t43323: f64, t999: f64, t16551: f64, t994: f64, t16558: f64, t16505: f64, t11627: f64, t11631: f64, t1043: f64, t11804: f64, t11940: f64, t12079: f64, t12086: f64, t12089: f64, t12111: f64, t12116: f64, t12119: f64, t12149: f64, t12160: f64, t3223: f64, t42001: f64, t42097: f64, t42615: f64, t43342: f64, t4996: f64, t4998: f64, t11620: f64, t4982: f64, t16553: f64, t12077: f64, t12047: f64, t12052: f64, t12074: f64, t12080: f64, t12131: f64, t12143: f64, t12146: f64, t12154: f64, t12157: f64, t16552: f64, t3291: f64, t3317: f64, t42047: f64, t42804: f64, t4981: f64, t12153: f64, t3316: f64, t1093: f64, t11687: f64, t11902: f64, t12057: f64, t12078: f64, t12094: f64, t12127: f64, t15604: f64, t16506: f64, t16523: f64, t3278: f64, t3302: f64, t3319: f64, t3322: f64, t43334: f64, t42358: f64, t1076: f64, t1079: f64, t1096: f64, t11173: f64, t11190: f64, t11201: f64, t11202: f64, t11203: f64, t11207: f64, t11210: f64, t11214: f64, t11220: f64, t11224: f64, t12043: f64, t12173: f64, t12174: f64, t225: f64, t3047: f64, t3058: f64, t3060: f64, t3067: f64, t3076: f64, t3264: f64, t3271: f64, t3326: f64, t385: f64, t43374: f64, t43409: f64, t43437: f64, t995: f64, t996: f64, t42277: f64, t1000: f64, t1097: f64, t11121: f64, t11123: f64, t11174: f64, t11187: f64, t11195: f64, t12034: f64, t3063: f64, t3269: f64, t3270: f64, t3325: f64, t1102: f64, t198: f64, t3336: f64, t336: f64, t41864: f64, t41867: f64, t41871: f64, t41873: f64, t41876: f64, t41879: f64, t41882: f64, t41885: f64, t41888: f64, t41947: f64, t41949: f64, t41950: f64, t42000: f64, t42112: f64, t30: f64, t265: f64, t393: f64, t41211: f64, t41477: f64, t41574: f64, t41943: f64, t10326: f64, t1106: f64, t11095: f64, t12201: f64, t2257: f64, t2258: f64, t2838: f64, t3340: f64, t39456: f64, t39457: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, t9344: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t3376: f64, t3383: f64, t3386: f64, t3494: f64, t3519: f64, t3497: f64, t1196: f64, t12555: f64, t12564: f64, t3531: f64, t12571: f64, t3543: f64, t12258: f64, t698: f64, t13026: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43438, t43439, t43443, t43446, t43450, t43453, t43456, t43467) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1547(t3057, t4980, t3059, t3151, t11223, t3286, t11200, t11213, t3046, t4995, t1071, t11247);
        let t43480 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1548(t3143, t42859, t342, t3154, t43351, t1089, t11788, t12066, t12073, t12128, t12150, t12163, t12167, t12168, t3059, t3204, t3287, t3288, t3304, t3318, t42610, t43292, t43348, t43438, t43439, t43443, t43446, t43450, t43453, t43456, t43467, t4976, t989);
        let (t43497, t43519) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1549(t3151, t3259, t12032, t359, t3043, t3298, t1024, t1082, t1087, t1089, t12122, t12132, t12133, t16410, t16520, t3075, t3118, t3133, t3153, t3204, t3287, t3299, t3304, t3305, t342, t378, t380, t42760, t42852, t42909, t43323, t999);
        let t43558 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1550(t16551, t994, t16558, t16505, t11627, t42859, t342, t11631, t43351, t1024, t1043, t1082, t1089, t11788, t11804, t11940, t12079, t12086, t12089, t12111, t12116, t12119, t12128, t12149, t12160, t12168, t3223, t42001, t42097, t42615, t43342, t43348, t4996, t4998);
        let t43593 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1551(t11620, t4982, t16553, t3133, t12077, t989, t1082, t1087, t1089, t11804, t12047, t12052, t12074, t12080, t12131, t12143, t12146, t12154, t12157, t16552, t3204, t3223, t3259, t3291, t3317, t3318, t42047, t42804, t43467, t43497, t4981);
        let t43626 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1552(t12153, t3057, t3043, t3316, t1071, t1087, t1089, t1093, t11620, t11687, t11902, t12057, t12078, t12079, t12094, t12127, t12143, t12150, t12154, t15604, t16506, t16523, t3278, t3299, t3302, t3304, t3319, t3322, t43334, t43467);
        let t43667 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1553(t1071, t11200, t378, t42358, t11223, t1076, t1079, t1096, t11173, t11190, t11201, t11202, t11203, t11207, t11210, t11214, t11220, t11224, t12043, t12173, t12174, t225, t3047, t3058, t3060, t3067, t3076, t3264, t3271, t3326, t342, t385, t42909, t43323, t43374, t43409, t43437, t43480, t43519, t43558, t43593, t43626, t995, t996, t999);
        let t43714 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1554(t12032, t994, t3259, t989, t1071, t11213, t378, t42277, t1000, t1076, t1096, t1097, t11121, t11123, t11174, t11187, t11195, t11207, t11210, t11214, t12034, t12043, t12173, t3047, t3058, t3059, t3063, t3067, t3075, t3264, t3269, t3270, t3271, t3325, t3326, t995);
        let t43720 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1555(t1102, t198, t3336, t336, t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41947, t41949, t41950, t42000, t42112, t43667, t43714);
        let t43735 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1556(t30, t265, t393, t41211, t41477, t41574, t41943, t43720, t10326, t1106, t11095, t12201, t2257, t2258, t2838, t3340, t39456, t39457, t395, t45, t605, t606, t895, t9344, dens_threshold, rho0, zeta_threshold);
        let (t43744, t43750, t43752, t43753) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1557(t39456, t3376, t3383, t3386, t3494, t3519, t3497);
        let (t43757, t43759, t43761, t43762, t43764) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1558(t1196, t12555, t43752, t43753, t12564, t3531, t12571, t3543, t12258, t698, t13026, t240);
    (t43735, t43744, t43750, t43752, t43753, t43757, t43759, t43761, t43762, t43764)
}
