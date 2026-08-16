//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta818 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3011;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3012;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3013;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3014;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3015;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3016;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3017;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3018;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3019;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta818(t12047: f64, t53552: f64, t15810: f64, t3127: f64, t3172: f64, t1063: f64, t11262: f64, t4802: f64, t4807: f64, t11859: f64, t11922: f64, t15894: f64, t1066: f64, t11698: f64, t11707: f64, t11977: f64, t15618: f64, t15850: f64, t16070: f64, t247: f64, t3177: f64, t43172: f64, t4869: f64, t51969: f64, t11714: f64, t4817: f64, t12004: f64, t1042: f64, t1045: f64, t11656: f64, t11774: f64, t15691: f64, t15847: f64, t16167: f64, t2858: f64, t3188: f64, t43204: f64, t43211: f64, t43215: f64, t43244: f64, t4788: f64, t4801: f64, t51958: f64, t53464: f64, t53474: f64, t999: f64, t3299: f64, t53401: f64, t16103: f64, t53405: f64, t16170: f64, t372: f64, t12116: f64, t15688: f64, t11145: f64, t11666: f64, t11706: f64, t11852: f64, t11994: f64, t12003: f64, t13312: f64, t15696: f64, t15811: f64, t16226: f64, t16229: f64, t16230: f64, t1651: f64, t3059: f64, t3204: f64, t42328: f64, t43069: f64, t4839: f64, t4872: f64, t53545: f64, t606: f64, t905: f64, t11773: f64, t15925: f64, t11783: f64, t4845: f64, t15745: f64, t3215: f64, t11792: f64, t15749: f64, t3224: f64, t11776: f64, t11866: f64, t15922: f64, t16186: f64, t1665: f64, t3169: f64, t42290: f64, t42355: f64, t43038: f64, t43238: f64, t4907: f64, t1043: f64, t15648: f64, t16039: f64, t3115: f64, t15610: f64, t1032: f64, t1040: f64, t15886: f64, t1011: f64, t1012: f64, t1015: f64, t1047: f64, t11173: f64, t11231: f64, t16123: f64, t3092: f64, t3117: f64, t3241: f64, t357: f64, t43242: f64, t43266: f64, t43277: f64, t4573: f64, t4781: f64, t49889: f64, t55011: f64, t3140: f64, t4743: f64, t3149: f64, t3160: f64, t15690: f64, t3153: f64, t11921: f64, t15716: f64, t15717: f64, t11804: f64, t11871: f64, t15758: f64, t15782: f64, t15817: f64, t1592: f64, t15926: f64, t16089: f64, t16102: f64, t16205: f64, t3136: f64, t3157: f64, t3164: f64, t43082: f64, t4823: f64, t4894: f64, t4900: f64, t1041: f64, t1670: f64, t42994: f64, t15786: f64, t4892: f64, t11779: f64, t11933: f64, t12160: f64, t15780: f64, t15787: f64, t16020: f64, t16040: f64, t16048: f64, t16052: f64, t16067: f64, t16068: f64, t16104: f64, t42340: f64, t43066: f64, t4806: f64, t4854: f64, t4899: f64, t4902: f64, t54450: f64, t54479: f64, t11988: f64, t4834: f64, t15731: f64, t3124: f64, t15794: f64, t42793: f64, t4911: f64, t1062: f64, t11637: f64, t15139: f64, t15957: f64, t16043: f64, t20094: f64, t42359: f64, t42410: f64, t43288: f64, t54909: f64, t11951: f64, t4858: f64, t15906: f64, t15909: f64, t16069: f64, t11200: f64, t380: f64, t16088: f64, t1025: f64, t11623: f64, t15651: f64, t15785: f64, t15895: f64, t16017: f64, t16049: f64, t371: f64, t373: f64, t42765: f64, t4879: f64, t53273: f64, t906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55046, t55058, t55062, t55065, t55067) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3011(t12047, t53552, t15810, t3127, t3172, t1063, t11262, t4802, t4807, t11859, t11922, t15894);
        let t55069 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3012(t1063, t1066, t11698, t11707, t11977, t15618, t15850, t16070, t247, t3177, t43172, t4869, t51969, t55046, t55058, t55062, t55065, t55067);
        let t55096 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3013(t11714, t4817, t12004, t1042, t1045, t1063, t11656, t11774, t15691, t15847, t16167, t2858, t3188, t43204, t43211, t43215, t43244, t4788, t4801, t51958, t53464, t53474, t999);
        let t55140 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3014(t3299, t53401, t11774, t16103, t53405, t16170, t372, t12116, t15688, t1042, t1045, t11145, t11666, t11706, t11852, t11994, t12003, t13312, t15691, t15696, t15811, t16226, t16229, t16230, t1651, t3059, t3127, t3204, t42328, t43069, t4839, t4872, t53545, t606, t905, t999);
        let t55163 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3015(t11773, t15925, t11783, t4845, t15745, t3215, t11792, t15749, t3224, t11776, t11866, t15922, t16186, t1665, t3169, t42290, t42355, t43038, t43238, t4907);
        let (t55165, t55198) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3016(t1043, t15648, t11922, t16039, t3115, t11859, t15610, t1032, t1040, t15886, t1011, t1012, t1015, t1045, t1047, t11173, t11231, t16123, t3092, t3117, t3241, t357, t43242, t43266, t43277, t4573, t4781, t49889, t55011);
        let t55237 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3017(t3140, t4743, t3149, t3160, t15690, t3153, t372, t11921, t15716, t15717, t247, t1043, t11804, t11871, t15758, t15782, t15817, t1592, t15926, t16089, t16102, t16103, t16205, t3092, t3136, t3157, t3164, t3188, t42328, t43069, t43082, t4823, t4894, t4900);
        let t55271 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3018(t1041, t1670, t42994, t11922, t15786, t4892, t1042, t1063, t11779, t11933, t12160, t15780, t15787, t16020, t16040, t16048, t16052, t16067, t16068, t16104, t1665, t3117, t42340, t43066, t4806, t4854, t4899, t4902, t54450, t54479);
        let t55303 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3019(t11988, t4834, t15731, t3124, t11933, t15794, t3115, t42793, t4911, t1062, t11231, t11637, t15139, t15782, t15957, t16043, t16052, t16089, t20094, t3092, t3117, t42359, t42410, t43288, t4839, t4892, t4894, t54909, t55011);
        let (t55330, t55338) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3020(t11951, t4858, t11922, t15906, t15909, t16067, t16069, t11200, t380, t16088, t1025, t11623, t11783, t15651, t15717, t15780, t15785, t15895, t16017, t16049, t3092, t3117, t3224, t371, t372, t373, t42765, t4854, t4879, t4892, t53273, t906);
    (t55069, t55096, t55140, t55163, t55165, t55198, t55237, t55271, t55303, t55330, t55338)
}
