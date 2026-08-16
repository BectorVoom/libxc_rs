//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1016 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3508;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3509;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3510;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3511;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3512;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3513;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3514;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1016(t11675: f64, t19785: f64, t1043: f64, t1045: f64, t15145: f64, t15691: f64, t15700: f64, t15895: f64, t15957: f64, t16017: f64, t16226: f64, t19501: f64, t19741: f64, t19776: f64, t19934: f64, t19998: f64, t3091: f64, t3092: f64, t3155: f64, t3188: f64, t42580: f64, t43175: f64, t4583: f64, t4892: f64, t53800: f64, t53993: f64, t53998: f64, t54026: f64, t55100: f64, t6266: f64, t11922: f64, t15906: f64, t19753: f64, t1011: f64, t1012: f64, t1015: f64, t11687: f64, t15689: f64, t19982: f64, t19993: f64, t23898: f64, t3117: f64, t42804: f64, t43050: f64, t43051: f64, t54014: f64, t54036: f64, t54039: f64, t54042: f64, t54047: f64, t54166: f64, t54801: f64, t55137: f64, t60754: f64, t20090: f64, t3115: f64, t19649: f64, t372: f64, t11774: f64, t20039: f64, t53405: f64, t19837: f64, t16068: f64, t16082: f64, t16095: f64, t16096: f64, t20075: f64, t3096: f64, t3151: f64, t42765: f64, t42872: f64, t43069: f64, t53676: f64, t54078: f64, t54081: f64, t54085: f64, t54316: f64, t54509: f64, t54811: f64, t6092: f64, t64891: f64, t6258: f64, t19744: f64, t20104: f64, t11703: f64, t11875: f64, t13396: f64, t15139: f64, t15153: f64, t15950: f64, t19750: f64, t19754: f64, t2852: f64, t3162: f64, t4181: f64, t42410: f64, t42656: f64, t4573: f64, t4772: f64, t53654: f64, t53657: f64, t54099: f64, t54118: f64, t54122: f64, t55011: f64, t15618: f64, t15984: f64, t42622: f64, t19477: f64, t73: f64, t15993: f64, t18913: f64, t18904: f64, t53972: f64, t11696: f64, t15601: f64, t15609: f64, t15615: f64, t16012: f64, t19450: f64, t19611: f64, t3095: f64, t42621: f64, t43105: f64, t4788: f64, t4919: f64, t54126: f64, t54578: f64, t63344: f64, t63357: f64, t15987: f64, t18942: f64, t15905: f64, t55599: f64, t905: f64, t11927: f64, t15599: f64, t16022: f64, t16070: f64, t16089: f64, t18946: f64, t19639: f64, t19641: f64, t19645: f64, t19836: f64, t19947: f64, t19951: f64, t3241: f64, t357: f64, t42690: f64, t42830: f64, t4786: f64, t53944: f64, t6100: f64, t6271: f64, t63349: f64, t64916: f64, t11710: f64, t19706: f64, t20095: f64, t16102: f64, t20079: f64, t42328: f64, t42710: f64, t43082: f64, t43085: f64, t4915: f64, t54142: f64, t54147: f64, t63244: f64, t63248: f64, t63306: f64, t63353: f64, t66187: f64, t13392: f64, t15787: f64, t15936: f64, t16020: f64, t16048: f64, t16052: f64, t16584: f64, t18941: f64, t19572: f64, t19738: f64, t20066: f64, t20094: f64, t20099: f64, t2857: f64, t42712: f64, t42716: f64, t42719: f64, t4899: f64, t4902: f64, t54023: f64, t54187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t66263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3508(t11675, t19785, t1043, t1045, t15145, t15691, t15700, t15895, t15957, t16017, t16226, t19501, t19741, t19776, t19934, t19998, t3091, t3092, t3155, t3188, t42580, t43175, t4583, t4892, t53800, t53993, t53998, t54026, t55100, t6266);
        let t66294 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3509(t11922, t15906, t19753, t1011, t1012, t1015, t11687, t15689, t15691, t19501, t19982, t19993, t19998, t23898, t3117, t42804, t43050, t43051, t54014, t54036, t54039, t54042, t54047, t54166, t54801, t55137, t60754, t6266);
        let t66336 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3510(t11922, t20090, t3115, t19649, t372, t11774, t20039, t53405, t19837, t15691, t16068, t16082, t16095, t16096, t20075, t3092, t3096, t3117, t3151, t42765, t42804, t42872, t43069, t53676, t54078, t54081, t54085, t54316, t54509, t54811, t6092, t6266, t64891);
        let (t66341, t66373) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3511(t3151, t6258, t11922, t19744, t3115, t20104, t11703, t11875, t13396, t15139, t15153, t15950, t16095, t19750, t19754, t2852, t3117, t3162, t4181, t42410, t42656, t4573, t4772, t53654, t53657, t54099, t54118, t54122, t55011);
        let (t66382, t66395, t66414) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3512(t15618, t15984, t1043, t42622, t19477, t73, t1011, t15993, t18913, t18904, t53972, t11696, t15601, t15609, t15615, t16012, t19450, t19611, t3091, t3092, t3095, t3117, t42621, t43105, t4788, t4919, t54126, t54578, t63344, t63357);
        let t66460 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3513(t1011, t15987, t18942, t15905, t55599, t6258, t905, t1045, t11675, t11875, t11927, t15599, t16022, t16070, t16089, t16096, t18946, t19639, t19641, t19645, t19741, t19836, t19947, t19951, t3091, t3092, t3117, t3241, t357, t42690, t42804, t42830, t4786, t53944, t6100, t6271, t63349, t64916);
        let t66500 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3514(t11710, t16089, t19706, t16095, t20095, t1011, t11675, t11703, t15599, t16102, t20079, t3091, t3162, t42328, t42710, t43082, t43085, t4915, t4919, t54142, t54147, t6092, t63244, t63248, t63306, t63353, t66187);
        let t66535 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3515(t13392, t15787, t15936, t16020, t16048, t16052, t16095, t16096, t16584, t18941, t19572, t19738, t19754, t20066, t20094, t20099, t2857, t3092, t3117, t4181, t42712, t42716, t42719, t4772, t4899, t4902, t54023, t54187);
    (t66263, t66294, t66336, t66341, t66373, t66382, t66395, t66414, t66460, t66500, t66535)
}
