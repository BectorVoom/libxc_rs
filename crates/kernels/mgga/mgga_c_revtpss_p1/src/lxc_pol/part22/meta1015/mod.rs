//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1015 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3500;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3501;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3502;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3503;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3504;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3505;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3506;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1015<F: Float>(t19697: F, t3173: F, t1042: F, t1063: F, t11703: F, t13392: F, t15725: F, t15758: F, t15935: F, t16095: F, t16096: F, t18903: F, t19651: F, t19663: F, t19688: F, t19800: F, t19973: F, t20099: F, t2853: F, t3059: F, t3106: F, t3127: F, t3169: F, t3181: F, t42410: F, t4837: F, t4872: F, t51963: F, t53661: F, t5825: F, t6258: F, t65370: F, t65947: F, t1041: F, t19799: F, t3172: F, t11262: F, t6301: F, t11999: F, t19826: F, t3150: F, t6307: F, t5819: F, t11710: F, t19725: F, t4892: F, t15669: F, t16088: F, t380: F, t11994: F, t15707: F, t16091: F, t16144: F, t18908: F, t19672: F, t19693: F, t4801: F, t51958: F, t53690: F, t1043: F, t54397: F, t1045: F, t4186: F, t53585: F, t1066: F, t11859: F, t15696: F, t15700: F, t15974: F, t16048: F, t16222: F, t16509: F, t19501: F, t247: F, t3117: F, t42328: F, t42450: F, t42481: F, t43116: F, t4896: F, t53710: F, t53724: F, t53762: F, t53771: F, t53790: F, t54658: F, t63330: F, t1058: F, t19858: F, t15688: F, t1053: F, t11632: F, t15604: F, t15691: F, t1592: F, t15973: F, t16226: F, t16230: F, t19450: F, t19857: F, t225: F, t3133: F, t3151: F, t3155: F, t366: F, t375: F, t42690: F, t4899: F, t53320: F, t53322: F, t53332: F, t53741: F, t53805: F, t53810: F, t53820: F, t6092: F, t60927: F, t65057: F, t4181: F, t19869: F, t3201: F, t6318: F, t1011: F, t15987: F, t18926: F, t18930: F, t19957: F, t19960: F, t19963: F, t3230: F, t3241: F, t43174: F, t4915: F, t53328: F, t53832: F, t53859: F, t53875: F, t55209: F, t6317: F, t63313: F, t15689: F, t19985: F, t53405: F, t1065: F, t372: F, t6305: F, t1012: F, t12131: F, t15130: F, t15135: F, t15701: F, t16228: F, t19776: F, t19980: F, t19993: F, t3253: F, t53728: F, t53881: F, t53898: F, t53901: F, t53923: F, t60717: F, t63236: F, t19912: F, t6292: F, t697: F, t11922: F, t19717: F, t11883: F, t16147: F, t19705: F, t3092: F, t53948: F, t53955: F, t53958: F, t53961: F, t53964: F, t53967: F, t53970: F, t53974: F, t55331: F, t6293: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t66013 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3500::<F>(t19697, t3173, t1042, t1063, t11703, t13392, t15725, t15758, t15935, t16095, t16096, t18903, t19651, t19663, t19688, t19800, t19973, t20099, t2853, t3059, t3106, t3127, t3169, t3181, t42410, t4837, t4872, t51963, t53661, t5825, t6258, t65370, t65947);
        let (t66017, t66022, t66024, t66029, t66037, t66043) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3501::<F>(t1041, t19799, t3172, t11262, t6301, t11999, t19826, t3150, t6307, t3059, t5819, t11710, t19725, t4892);
        let t66054 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3502::<F>(t15669, t16088, t380, t1042, t1063, t11703, t11994, t15707, t16091, t16095, t16096, t16144, t18908, t19672, t19693, t3106, t4801, t4837, t51958, t53690, t65947, t66017, t66022, t66024, t66029, t66037, t66043);
        let (t66061, t66062, t66067, t66086) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3503::<F>(t1043, t5819, t54397, t1045, t4186, t53585, t1063, t1066, t11859, t15696, t15700, t15974, t16048, t16222, t16509, t19501, t247, t3117, t42328, t42450, t42481, t43116, t4896, t53710, t53724, t53762, t53771, t53790, t54658, t63330);
        let t66127 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3504::<F>(t1058, t19858, t15688, t16509, t1053, t11632, t11703, t15604, t15691, t1592, t15973, t16226, t16230, t19450, t19857, t225, t3117, t3133, t3151, t3155, t366, t375, t42690, t4899, t53320, t53322, t53332, t53741, t53805, t53810, t53820, t6092, t60927, t65057);
        let (t66128, t66161) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3505::<F>(t1043, t4181, t1058, t19869, t3201, t6318, t1011, t15987, t18926, t18930, t16226, t19957, t19960, t19963, t3230, t3241, t375, t43174, t4915, t53320, t53328, t53832, t53859, t53875, t55209, t60927, t6317, t63313);
        let (t66187, t66204) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3506::<F>(t15689, t19985, t53405, t1065, t372, t6305, t1011, t1012, t1045, t11632, t12131, t15130, t15135, t15691, t15700, t15701, t16228, t19776, t19980, t19993, t3253, t4915, t53728, t53741, t53881, t53898, t53901, t53923, t60717, t63236, t66062, t66067);
        let t66227 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3507::<F>(t19912, t3241, t1011, t6292, t697, t11922, t19717, t4899, t11883, t16147, t19705, t3092, t53948, t53955, t53958, t53961, t53964, t53967, t53970, t53974, t55331, t6293);
    (t66013, t66037, t66054, t66061, t66062, t66086, t66127, t66128, t66161, t66187, t66204, t66227)
}
