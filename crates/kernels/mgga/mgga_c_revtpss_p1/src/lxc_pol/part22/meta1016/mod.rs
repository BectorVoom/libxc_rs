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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3508;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3509;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3510;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3511;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3512;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3513;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3514;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1016<F: Float>(t11675: F, t19785: F, t1043: F, t1045: F, t15145: F, t15691: F, t15700: F, t15895: F, t15957: F, t16017: F, t16226: F, t19501: F, t19741: F, t19776: F, t19934: F, t19998: F, t3091: F, t3092: F, t3155: F, t3188: F, t42580: F, t43175: F, t4583: F, t4892: F, t53800: F, t53993: F, t53998: F, t54026: F, t55100: F, t6266: F, t11922: F, t15906: F, t19753: F, t1011: F, t1012: F, t1015: F, t11687: F, t15689: F, t19982: F, t19993: F, t23898: F, t3117: F, t42804: F, t43050: F, t43051: F, t54014: F, t54036: F, t54039: F, t54042: F, t54047: F, t54166: F, t54801: F, t55137: F, t60754: F, t20090: F, t3115: F, t19649: F, t372: F, t11774: F, t20039: F, t53405: F, t19837: F, t16068: F, t16082: F, t16095: F, t16096: F, t20075: F, t3096: F, t3151: F, t42765: F, t42872: F, t43069: F, t53676: F, t54078: F, t54081: F, t54085: F, t54316: F, t54509: F, t54811: F, t6092: F, t64891: F, t6258: F, t19744: F, t20104: F, t11703: F, t11875: F, t13396: F, t15139: F, t15153: F, t15950: F, t19750: F, t19754: F, t2852: F, t3162: F, t4181: F, t42410: F, t42656: F, t4573: F, t4772: F, t53654: F, t53657: F, t54099: F, t54118: F, t54122: F, t55011: F, t15618: F, t15984: F, t42622: F, t19477: F, t73: F, t15993: F, t18913: F, t18904: F, t53972: F, t11696: F, t15601: F, t15609: F, t15615: F, t16012: F, t19450: F, t19611: F, t3095: F, t42621: F, t43105: F, t4788: F, t4919: F, t54126: F, t54578: F, t63344: F, t63357: F, t15987: F, t18942: F, t15905: F, t55599: F, t905: F, t11927: F, t15599: F, t16022: F, t16070: F, t16089: F, t18946: F, t19639: F, t19641: F, t19645: F, t19836: F, t19947: F, t19951: F, t3241: F, t357: F, t42690: F, t42830: F, t4786: F, t53944: F, t6100: F, t6271: F, t63349: F, t64916: F, t11710: F, t19706: F, t20095: F, t16102: F, t20079: F, t42328: F, t42710: F, t43082: F, t43085: F, t4915: F, t54142: F, t54147: F, t63244: F, t63248: F, t63306: F, t63353: F, t66187: F, t13392: F, t15787: F, t15936: F, t16020: F, t16048: F, t16052: F, t16584: F, t18941: F, t19572: F, t19738: F, t20066: F, t20094: F, t20099: F, t2857: F, t42712: F, t42716: F, t42719: F, t4899: F, t4902: F, t54023: F, t54187: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t66263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3508::<F>(t11675, t19785, t1043, t1045, t15145, t15691, t15700, t15895, t15957, t16017, t16226, t19501, t19741, t19776, t19934, t19998, t3091, t3092, t3155, t3188, t42580, t43175, t4583, t4892, t53800, t53993, t53998, t54026, t55100, t6266);
        let t66294 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3509::<F>(t11922, t15906, t19753, t1011, t1012, t1015, t11687, t15689, t15691, t19501, t19982, t19993, t19998, t23898, t3117, t42804, t43050, t43051, t54014, t54036, t54039, t54042, t54047, t54166, t54801, t55137, t60754, t6266);
        let t66336 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3510::<F>(t11922, t20090, t3115, t19649, t372, t11774, t20039, t53405, t19837, t15691, t16068, t16082, t16095, t16096, t20075, t3092, t3096, t3117, t3151, t42765, t42804, t42872, t43069, t53676, t54078, t54081, t54085, t54316, t54509, t54811, t6092, t6266, t64891);
        let (t66341, t66373) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3511::<F>(t3151, t6258, t11922, t19744, t3115, t20104, t11703, t11875, t13396, t15139, t15153, t15950, t16095, t19750, t19754, t2852, t3117, t3162, t4181, t42410, t42656, t4573, t4772, t53654, t53657, t54099, t54118, t54122, t55011);
        let (t66382, t66395, t66414) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3512::<F>(t15618, t15984, t1043, t42622, t19477, t73, t1011, t15993, t18913, t18904, t53972, t11696, t15601, t15609, t15615, t16012, t19450, t19611, t3091, t3092, t3095, t3117, t42621, t43105, t4788, t4919, t54126, t54578, t63344, t63357);
        let t66460 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3513::<F>(t1011, t15987, t18942, t15905, t55599, t6258, t905, t1045, t11675, t11875, t11927, t15599, t16022, t16070, t16089, t16096, t18946, t19639, t19641, t19645, t19741, t19836, t19947, t19951, t3091, t3092, t3117, t3241, t357, t42690, t42804, t42830, t4786, t53944, t6100, t6271, t63349, t64916);
        let t66500 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3514::<F>(t11710, t16089, t19706, t16095, t20095, t1011, t11675, t11703, t15599, t16102, t20079, t3091, t3162, t42328, t42710, t43082, t43085, t4915, t4919, t54142, t54147, t6092, t63244, t63248, t63306, t63353, t66187);
        let t66535 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3515::<F>(t13392, t15787, t15936, t16020, t16048, t16052, t16095, t16096, t16584, t18941, t19572, t19738, t19754, t20066, t20094, t20099, t2857, t3092, t3117, t4181, t42712, t42716, t42719, t4772, t4899, t4902, t54023, t54187);
    (t66263, t66294, t66336, t66341, t66373, t66382, t66395, t66414, t66460, t66500, t66535)
}
