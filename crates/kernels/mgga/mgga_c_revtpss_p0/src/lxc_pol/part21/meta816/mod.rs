//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta816 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2995;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2996;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2997;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2998;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2999;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3000;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3001;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3002;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3003;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta816<F: Float>(t11656: F, t15769: F, t16199: F, t372: F, t127: F, t15700: F, t15702: F, t4806: F, t16208: F, t15666: F, t3211: F, t15656: F, t3215: F, t11806: F, t15689: F, t15701: F, t15745: F, t16222: F, t19878: F, t3220: F, t42900: F, t53729: F, t53735: F, t54263: F, t1025: F, t1663: F, t2434: F, t371: F, t15649: F, t225: F, t53166: F, t366: F, t1053: F, t15655: F, t3224: F, t11991: F, t4817: F, t1028: F, t11792: F, t15651: F, t1665: F, t3208: F, t42279: F, t42902: F, t42907: F, t4854: F, t15731: F, t3169: F, t11281: F, t11774: F, t11883: F, t1469: F, t15707: F, t15725: F, t15804: F, t16149: F, t3241: F, t42926: F, t42929: F, t42932: F, t42947: F, t42962: F, t4801: F, t4916: F, t54398: F, t15816: F, t3168: F, t10326: F, t10356: F, t1047: F, t11144: F, t11675: F, t15599: F, t15601: F, t15622: F, t1592: F, t3091: F, t3092: F, t3094: F, t3095: F, t357: F, t42410: F, t42610: F, t42965: F, t42996: F, t43003: F, t43297: F, t4583: F, t4781: F, t54026: F, t11710: F, t15591: F, t16060: F, t1011: F, t140: F, t16122: F, t12078: F, t53740: F, t11661: F, t11684: F, t11696: F, t11927: F, t12131: F, t15618: F, t15691: F, t15717: F, t15957: F, t16025: F, t16190: F, t19980: F, t3117: F, t3136: F, t42316: F, t42804: F, t43291: F, t4786: F, t4887: F, t12047: F, t16138: F, t16158: F, t3106: F, t12003: F, t1659: F, t11648: F, t4879: F, t1042: F, t1068: F, t11286: F, t11679: F, t11705: F, t11983: F, t11994: F, t15696: F, t15697: F, t16140: F, t3075: F, t3096: F, t3127: F, t4186: F, t42155: F, t4834: F, t4872: F, t1063: F, t15790: F, t3172: F, t11223: F, t16088: F, t380: F, t1041: F, t16185: F, t11202: F, t11637: F, t11933: F, t15139: F, t16078: F, t16091: F, t16095: F, t42571: F, t43017: F, t43019: F, t43057: F, t4573: F, t4875: F, t42415: F, t4890: F, t1062: F, t42261: F, t11913: F, t15719: F, t15850: F, t15975: F, t16049: F, t3101: F, t3299: F, t3317: F, t43029: F, t43032: F, t43035: F, t43038: F, t43121: F, t4896: F, t4902: F, t4912: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54656, t54658, t54667, t54672, t54678, t54680) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2995::<F>(t11656, t15769, t16199, t372, t127, t15700, t15702, t4806, t16208, t15666, t3211, t15656, t3215);
        let t54684 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2996::<F>(t11806, t15689, t15700, t15701, t15745, t16222, t19878, t3220, t42900, t53729, t53735, t54263, t54656, t54658, t54667, t54672, t54678, t54680);
        let (t54687, t54693, t54695, t54696, t54699, t54704) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2997::<F>(t1025, t1663, t2434, t371, t127, t15649, t225, t53166, t366, t1053, t15655, t15666, t3224);
        let t54712 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2998::<F>(t11991, t4817, t1028, t11792, t15651, t1665, t3208, t3211, t42279, t42902, t42907, t4854, t54687, t54693, t54696, t54699, t54704);
        let t54735 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2999::<F>(t15731, t3169, t11281, t11774, t11883, t1469, t15707, t15725, t15804, t16149, t3241, t372, t42926, t42929, t42932, t42947, t42962, t4801, t4916, t54398);
        let t54770 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3000::<F>(t15816, t3168, t10326, t10356, t1047, t11144, t11675, t15599, t15601, t15622, t1592, t3091, t3092, t3094, t3095, t357, t42410, t42610, t42965, t42996, t43003, t43297, t4583, t4781, t54026);
        let t54806 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3001::<F>(t11710, t15591, t3091, t16060, t3241, t1011, t140, t16122, t12078, t53740, t11661, t11684, t11696, t11883, t11927, t12131, t15618, t15689, t15691, t15700, t15717, t15957, t16025, t16190, t19980, t3117, t3136, t42316, t42804, t43291, t4786, t4887);
        let t54843 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3002::<F>(t12047, t53740, t16138, t372, t16158, t3106, t12003, t1659, t11648, t4879, t1042, t1068, t11286, t11679, t11705, t11774, t11983, t11994, t12131, t15689, t15691, t15696, t15697, t15707, t16140, t19980, t3075, t3095, t3096, t3127, t4186, t42155, t42804, t4834, t4872);
        let t54880 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3003::<F>(t1063, t15790, t3172, t11223, t16088, t380, t1041, t16185, t11202, t11637, t11774, t11933, t15139, t16078, t16091, t16095, t19980, t3092, t3117, t357, t42410, t42571, t43017, t43019, t43057, t43291, t4573, t4781, t4875);
        let t54904 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3004::<F>(t42415, t4890, t1062, t42261, t11913, t15719, t15850, t15975, t16049, t3101, t3299, t3317, t43029, t43032, t43035, t43038, t43121, t4834, t4896, t4902, t4912);
    (t54684, t54695, t54712, t54735, t54770, t54806, t54843, t54880, t54904)
}
