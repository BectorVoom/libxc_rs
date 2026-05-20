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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta416<F: Float>(t3057: F, t4980: F, t3059: F, t3151: F, t11223: F, t3286: F, t11200: F, t11213: F, t3046: F, t4995: F, t1071: F, t11247: F, t3143: F, t42859: F, t342: F, t3154: F, t43351: F, t1089: F, t11788: F, t12066: F, t12073: F, t12128: F, t12150: F, t12163: F, t12167: F, t12168: F, t3204: F, t3287: F, t3288: F, t3304: F, t3318: F, t42610: F, t43292: F, t43348: F, t4976: F, t989: F, t3259: F, t12032: F, t359: F, t3043: F, t3298: F, t1024: F, t1082: F, t1087: F, t12122: F, t12132: F, t12133: F, t16410: F, t16520: F, t3075: F, t3118: F, t3133: F, t3153: F, t3299: F, t3305: F, t378: F, t380: F, t42760: F, t42852: F, t42909: F, t43323: F, t999: F, t16551: F, t994: F, t16558: F, t16505: F, t11627: F, t11631: F, t1043: F, t11804: F, t11940: F, t12079: F, t12086: F, t12089: F, t12111: F, t12116: F, t12119: F, t12149: F, t12160: F, t3223: F, t42001: F, t42097: F, t42615: F, t43342: F, t4996: F, t4998: F, t11620: F, t4982: F, t16553: F, t12077: F, t12047: F, t12052: F, t12074: F, t12080: F, t12131: F, t12143: F, t12146: F, t12154: F, t12157: F, t16552: F, t3291: F, t3317: F, t42047: F, t42804: F, t4981: F, t12153: F, t3316: F, t1093: F, t11687: F, t11902: F, t12057: F, t12078: F, t12094: F, t12127: F, t15604: F, t16506: F, t16523: F, t3278: F, t3302: F, t3319: F, t3322: F, t43334: F, t42358: F, t1076: F, t1079: F, t1096: F, t11173: F, t11190: F, t11201: F, t11202: F, t11203: F, t11207: F, t11210: F, t11214: F, t11220: F, t11224: F, t12043: F, t12173: F, t12174: F, t225: F, t3047: F, t3058: F, t3060: F, t3067: F, t3076: F, t3264: F, t3271: F, t3326: F, t385: F, t43374: F, t43409: F, t43437: F, t995: F, t996: F, t42277: F, t1000: F, t1097: F, t11121: F, t11123: F, t11174: F, t11187: F, t11195: F, t12034: F, t3063: F, t3269: F, t3270: F, t3325: F, t1102: F, t198: F, t3336: F, t336: F, t41864: F, t41867: F, t41871: F, t41873: F, t41876: F, t41879: F, t41882: F, t41885: F, t41888: F, t41947: F, t41949: F, t41950: F, t42000: F, t42112: F, t30: F, t265: F, t393: F, t41211: F, t41477: F, t41574: F, t41943: F, t10326: F, t1106: F, t11095: F, t12201: F, t2257: F, t2258: F, t2838: F, t3340: F, t39456: F, t39457: F, t395: F, t45: F, t605: F, t606: F, t895: F, t9344: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3376: F, t3383: F, t3386: F, t3494: F, t3519: F, t3497: F, t1196: F, t12555: F, t12564: F, t3531: F, t12571: F, t3543: F, t12258: F, t698: F, t13026: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43438, t43439, t43443, t43446, t43450, t43453, t43456, t43467) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1547::<F>(t3057, t4980, t3059, t3151, t11223, t3286, t11200, t11213, t3046, t4995, t1071, t11247);
        let t43480 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1548::<F>(t3143, t42859, t342, t3154, t43351, t1089, t11788, t12066, t12073, t12128, t12150, t12163, t12167, t12168, t3059, t3204, t3287, t3288, t3304, t3318, t42610, t43292, t43348, t43438, t43439, t43443, t43446, t43450, t43453, t43456, t43467, t4976, t989);
        let (t43497, t43519) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1549::<F>(t3151, t3259, t12032, t359, t3043, t3298, t1024, t1082, t1087, t1089, t12122, t12132, t12133, t16410, t16520, t3075, t3118, t3133, t3153, t3204, t3287, t3299, t3304, t3305, t342, t378, t380, t42760, t42852, t42909, t43323, t999);
        let t43558 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1550::<F>(t16551, t994, t16558, t16505, t11627, t42859, t342, t11631, t43351, t1024, t1043, t1082, t1089, t11788, t11804, t11940, t12079, t12086, t12089, t12111, t12116, t12119, t12128, t12149, t12160, t12168, t3223, t42001, t42097, t42615, t43342, t43348, t4996, t4998);
        let t43593 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1551::<F>(t11620, t4982, t16553, t3133, t12077, t989, t1082, t1087, t1089, t11804, t12047, t12052, t12074, t12080, t12131, t12143, t12146, t12154, t12157, t16552, t3204, t3223, t3259, t3291, t3317, t3318, t42047, t42804, t43467, t43497, t4981);
        let t43626 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1552::<F>(t12153, t3057, t3043, t3316, t1071, t1087, t1089, t1093, t11620, t11687, t11902, t12057, t12078, t12079, t12094, t12127, t12143, t12150, t12154, t15604, t16506, t16523, t3278, t3299, t3302, t3304, t3319, t3322, t43334, t43467);
        let t43667 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1553::<F>(t1071, t11200, t378, t42358, t11223, t1076, t1079, t1096, t11173, t11190, t11201, t11202, t11203, t11207, t11210, t11214, t11220, t11224, t12043, t12173, t12174, t225, t3047, t3058, t3060, t3067, t3076, t3264, t3271, t3326, t342, t385, t42909, t43323, t43374, t43409, t43437, t43480, t43519, t43558, t43593, t43626, t995, t996, t999);
        let t43714 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1554::<F>(t12032, t994, t3259, t989, t1071, t11213, t378, t42277, t1000, t1076, t1096, t1097, t11121, t11123, t11174, t11187, t11195, t11207, t11210, t11214, t12034, t12043, t12173, t3047, t3058, t3059, t3063, t3067, t3075, t3264, t3269, t3270, t3271, t3325, t3326, t995);
        let t43720 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1555::<F>(t1102, t198, t3336, t336, t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41947, t41949, t41950, t42000, t42112, t43667, t43714);
        let t43735 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1556::<F>(t30, t265, t393, t41211, t41477, t41574, t41943, t43720, t10326, t1106, t11095, t12201, t2257, t2258, t2838, t3340, t39456, t39457, t395, t45, t605, t606, t895, t9344, dens_threshold, rho0, zeta_threshold);
        let (t43744, t43750, t43752, t43753) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1557::<F>(t39456, t3376, t3383, t3386, t3494, t3519, t3497);
        let (t43757, t43759, t43761, t43762, t43764) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1558::<F>(t1196, t12555, t43752, t43753, t12564, t3531, t12571, t3543, t12258, t698, t13026, t240);
    (t43735, t43744, t43750, t43752, t43753, t43757, t43759, t43761, t43762, t43764)
}
