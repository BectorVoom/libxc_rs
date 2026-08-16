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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3500;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3501;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3502;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3503;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3504;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3505;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3506;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1015(t19697: f64, t3173: f64, t1042: f64, t1063: f64, t11703: f64, t13392: f64, t15725: f64, t15758: f64, t15935: f64, t16095: f64, t16096: f64, t18903: f64, t19651: f64, t19663: f64, t19688: f64, t19800: f64, t19973: f64, t20099: f64, t2853: f64, t3059: f64, t3106: f64, t3127: f64, t3169: f64, t3181: f64, t42410: f64, t4837: f64, t4872: f64, t51963: f64, t53661: f64, t5825: f64, t6258: f64, t65370: f64, t65947: f64, t1041: f64, t19799: f64, t3172: f64, t11262: f64, t6301: f64, t11999: f64, t19826: f64, t3150: f64, t6307: f64, t5819: f64, t11710: f64, t19725: f64, t4892: f64, t15669: f64, t16088: f64, t380: f64, t11994: f64, t15707: f64, t16091: f64, t16144: f64, t18908: f64, t19672: f64, t19693: f64, t4801: f64, t51958: f64, t53690: f64, t1043: f64, t54397: f64, t1045: f64, t4186: f64, t53585: f64, t1066: f64, t11859: f64, t15696: f64, t15700: f64, t15974: f64, t16048: f64, t16222: f64, t16509: f64, t19501: f64, t247: f64, t3117: f64, t42328: f64, t42450: f64, t42481: f64, t43116: f64, t4896: f64, t53710: f64, t53724: f64, t53762: f64, t53771: f64, t53790: f64, t54658: f64, t63330: f64, t1058: f64, t19858: f64, t15688: f64, t1053: f64, t11632: f64, t15604: f64, t15691: f64, t1592: f64, t15973: f64, t16226: f64, t16230: f64, t19450: f64, t19857: f64, t225: f64, t3133: f64, t3151: f64, t3155: f64, t366: f64, t375: f64, t42690: f64, t4899: f64, t53320: f64, t53322: f64, t53332: f64, t53741: f64, t53805: f64, t53810: f64, t53820: f64, t6092: f64, t60927: f64, t65057: f64, t4181: f64, t19869: f64, t3201: f64, t6318: f64, t1011: f64, t15987: f64, t18926: f64, t18930: f64, t19957: f64, t19960: f64, t19963: f64, t3230: f64, t3241: f64, t43174: f64, t4915: f64, t53328: f64, t53832: f64, t53859: f64, t53875: f64, t55209: f64, t6317: f64, t63313: f64, t15689: f64, t19985: f64, t53405: f64, t1065: f64, t372: f64, t6305: f64, t1012: f64, t12131: f64, t15130: f64, t15135: f64, t15701: f64, t16228: f64, t19776: f64, t19980: f64, t19993: f64, t3253: f64, t53728: f64, t53881: f64, t53898: f64, t53901: f64, t53923: f64, t60717: f64, t63236: f64, t19912: f64, t6292: f64, t697: f64, t11922: f64, t19717: f64, t11883: f64, t16147: f64, t19705: f64, t3092: f64, t53948: f64, t53955: f64, t53958: f64, t53961: f64, t53964: f64, t53967: f64, t53970: f64, t53974: f64, t55331: f64, t6293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t66013 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3500(t19697, t3173, t1042, t1063, t11703, t13392, t15725, t15758, t15935, t16095, t16096, t18903, t19651, t19663, t19688, t19800, t19973, t20099, t2853, t3059, t3106, t3127, t3169, t3181, t42410, t4837, t4872, t51963, t53661, t5825, t6258, t65370, t65947);
        let (t66017, t66022, t66024, t66029, t66037, t66043) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3501(t1041, t19799, t3172, t11262, t6301, t11999, t19826, t3150, t6307, t3059, t5819, t11710, t19725, t4892);
        let t66054 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3502(t15669, t16088, t380, t1042, t1063, t11703, t11994, t15707, t16091, t16095, t16096, t16144, t18908, t19672, t19693, t3106, t4801, t4837, t51958, t53690, t65947, t66017, t66022, t66024, t66029, t66037, t66043);
        let (t66061, t66062, t66067, t66086) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3503(t1043, t5819, t54397, t1045, t4186, t53585, t1063, t1066, t11859, t15696, t15700, t15974, t16048, t16222, t16509, t19501, t247, t3117, t42328, t42450, t42481, t43116, t4896, t53710, t53724, t53762, t53771, t53790, t54658, t63330);
        let t66127 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3504(t1058, t19858, t15688, t16509, t1053, t11632, t11703, t15604, t15691, t1592, t15973, t16226, t16230, t19450, t19857, t225, t3117, t3133, t3151, t3155, t366, t375, t42690, t4899, t53320, t53322, t53332, t53741, t53805, t53810, t53820, t6092, t60927, t65057);
        let (t66128, t66161) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3505(t1043, t4181, t1058, t19869, t3201, t6318, t1011, t15987, t18926, t18930, t16226, t19957, t19960, t19963, t3230, t3241, t375, t43174, t4915, t53320, t53328, t53832, t53859, t53875, t55209, t60927, t6317, t63313);
        let (t66187, t66204) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3506(t15689, t19985, t53405, t1065, t372, t6305, t1011, t1012, t1045, t11632, t12131, t15130, t15135, t15691, t15700, t15701, t16228, t19776, t19980, t19993, t3253, t4915, t53728, t53741, t53881, t53898, t53901, t53923, t60717, t63236, t66062, t66067);
        let t66227 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3507(t19912, t3241, t1011, t6292, t697, t11922, t19717, t4899, t11883, t16147, t19705, t3092, t53948, t53955, t53958, t53961, t53964, t53967, t53970, t53974, t55331, t6293);
    (t66013, t66037, t66054, t66061, t66062, t66086, t66127, t66128, t66161, t66187, t66204, t66227)
}
