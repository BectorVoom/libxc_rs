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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta818<F: Float>(t12047: F, t53552: F, t15810: F, t3127: F, t3172: F, t1063: F, t11262: F, t4802: F, t4807: F, t11859: F, t11922: F, t15894: F, t1066: F, t11698: F, t11707: F, t11977: F, t15618: F, t15850: F, t16070: F, t247: F, t3177: F, t43172: F, t4869: F, t51969: F, t11714: F, t4817: F, t12004: F, t1042: F, t1045: F, t11656: F, t11774: F, t15691: F, t15847: F, t16167: F, t2858: F, t3188: F, t43204: F, t43211: F, t43215: F, t43244: F, t4788: F, t4801: F, t51958: F, t53464: F, t53474: F, t999: F, t3299: F, t53401: F, t16103: F, t53405: F, t16170: F, t372: F, t12116: F, t15688: F, t11145: F, t11666: F, t11706: F, t11852: F, t11994: F, t12003: F, t13312: F, t15696: F, t15811: F, t16226: F, t16229: F, t16230: F, t1651: F, t3059: F, t3204: F, t42328: F, t43069: F, t4839: F, t4872: F, t53545: F, t606: F, t905: F, t11773: F, t15925: F, t11783: F, t4845: F, t15745: F, t3215: F, t11792: F, t15749: F, t3224: F, t11776: F, t11866: F, t15922: F, t16186: F, t1665: F, t3169: F, t42290: F, t42355: F, t43038: F, t43238: F, t4907: F, t1043: F, t15648: F, t16039: F, t3115: F, t15610: F, t1032: F, t1040: F, t15886: F, t1011: F, t1012: F, t1015: F, t1047: F, t11173: F, t11231: F, t16123: F, t3092: F, t3117: F, t3241: F, t357: F, t43242: F, t43266: F, t43277: F, t4573: F, t4781: F, t49889: F, t55011: F, t3140: F, t4743: F, t3149: F, t3160: F, t15690: F, t3153: F, t11921: F, t15716: F, t15717: F, t11804: F, t11871: F, t15758: F, t15782: F, t15817: F, t1592: F, t15926: F, t16089: F, t16102: F, t16205: F, t3136: F, t3157: F, t3164: F, t43082: F, t4823: F, t4894: F, t4900: F, t1041: F, t1670: F, t42994: F, t15786: F, t4892: F, t11779: F, t11933: F, t12160: F, t15780: F, t15787: F, t16020: F, t16040: F, t16048: F, t16052: F, t16067: F, t16068: F, t16104: F, t42340: F, t43066: F, t4806: F, t4854: F, t4899: F, t4902: F, t54450: F, t54479: F, t11988: F, t4834: F, t15731: F, t3124: F, t15794: F, t42793: F, t4911: F, t1062: F, t11637: F, t15139: F, t15957: F, t16043: F, t20094: F, t42359: F, t42410: F, t43288: F, t54909: F, t11951: F, t4858: F, t15906: F, t15909: F, t16069: F, t11200: F, t380: F, t16088: F, t1025: F, t11623: F, t15651: F, t15785: F, t15895: F, t16017: F, t16049: F, t371: F, t373: F, t42765: F, t4879: F, t53273: F, t906: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t55046, t55058, t55062, t55065, t55067) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3011::<F>(t12047, t53552, t15810, t3127, t3172, t1063, t11262, t4802, t4807, t11859, t11922, t15894);
        let t55069 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3012::<F>(t1063, t1066, t11698, t11707, t11977, t15618, t15850, t16070, t247, t3177, t43172, t4869, t51969, t55046, t55058, t55062, t55065, t55067);
        let t55096 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3013::<F>(t11714, t4817, t12004, t1042, t1045, t1063, t11656, t11774, t15691, t15847, t16167, t2858, t3188, t43204, t43211, t43215, t43244, t4788, t4801, t51958, t53464, t53474, t999);
        let t55140 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3014::<F>(t3299, t53401, t11774, t16103, t53405, t16170, t372, t12116, t15688, t1042, t1045, t11145, t11666, t11706, t11852, t11994, t12003, t13312, t15691, t15696, t15811, t16226, t16229, t16230, t1651, t3059, t3127, t3204, t42328, t43069, t4839, t4872, t53545, t606, t905, t999);
        let t55163 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3015::<F>(t11773, t15925, t11783, t4845, t15745, t3215, t11792, t15749, t3224, t11776, t11866, t15922, t16186, t1665, t3169, t42290, t42355, t43038, t43238, t4907);
        let (t55165, t55198) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3016::<F>(t1043, t15648, t11922, t16039, t3115, t11859, t15610, t1032, t1040, t15886, t1011, t1012, t1015, t1045, t1047, t11173, t11231, t16123, t3092, t3117, t3241, t357, t43242, t43266, t43277, t4573, t4781, t49889, t55011);
        let t55237 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3017::<F>(t3140, t4743, t3149, t3160, t15690, t3153, t372, t11921, t15716, t15717, t247, t1043, t11804, t11871, t15758, t15782, t15817, t1592, t15926, t16089, t16102, t16103, t16205, t3092, t3136, t3157, t3164, t3188, t42328, t43069, t43082, t4823, t4894, t4900);
        let t55271 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3018::<F>(t1041, t1670, t42994, t11922, t15786, t4892, t1042, t1063, t11779, t11933, t12160, t15780, t15787, t16020, t16040, t16048, t16052, t16067, t16068, t16104, t1665, t3117, t42340, t43066, t4806, t4854, t4899, t4902, t54450, t54479);
        let t55303 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3019::<F>(t11988, t4834, t15731, t3124, t11933, t15794, t3115, t42793, t4911, t1062, t11231, t11637, t15139, t15782, t15957, t16043, t16052, t16089, t20094, t3092, t3117, t42359, t42410, t43288, t4839, t4892, t4894, t54909, t55011);
        let (t55330, t55338) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3020::<F>(t11951, t4858, t11922, t15906, t15909, t16067, t16069, t11200, t380, t16088, t1025, t11623, t11783, t15651, t15717, t15780, t15785, t15895, t16017, t16049, t3092, t3117, t3224, t371, t372, t373, t42765, t4854, t4879, t4892, t53273, t906);
    (t55069, t55096, t55140, t55163, t55165, t55198, t55237, t55271, t55303, t55330, t55338)
}
