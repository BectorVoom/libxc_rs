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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta816(t11656: f64, t15769: f64, t16199: f64, t372: f64, t127: f64, t15700: f64, t15702: f64, t4806: f64, t16208: f64, t15666: f64, t3211: f64, t15656: f64, t3215: f64, t11806: f64, t15689: f64, t15701: f64, t15745: f64, t16222: f64, t19878: f64, t3220: f64, t42900: f64, t53729: f64, t53735: f64, t54263: f64, t1025: f64, t1663: f64, t2434: f64, t371: f64, t15649: f64, t225: f64, t53166: f64, t366: f64, t1053: f64, t15655: f64, t3224: f64, t11991: f64, t4817: f64, t1028: f64, t11792: f64, t15651: f64, t1665: f64, t3208: f64, t42279: f64, t42902: f64, t42907: f64, t4854: f64, t15731: f64, t3169: f64, t11281: f64, t11774: f64, t11883: f64, t1469: f64, t15707: f64, t15725: f64, t15804: f64, t16149: f64, t3241: f64, t42926: f64, t42929: f64, t42932: f64, t42947: f64, t42962: f64, t4801: f64, t4916: f64, t54398: f64, t15816: f64, t3168: f64, t10326: f64, t10356: f64, t1047: f64, t11144: f64, t11675: f64, t15599: f64, t15601: f64, t15622: f64, t1592: f64, t3091: f64, t3092: f64, t3094: f64, t3095: f64, t357: f64, t42410: f64, t42610: f64, t42965: f64, t42996: f64, t43003: f64, t43297: f64, t4583: f64, t4781: f64, t54026: f64, t11710: f64, t15591: f64, t16060: f64, t1011: f64, t140: f64, t16122: f64, t12078: f64, t53740: f64, t11661: f64, t11684: f64, t11696: f64, t11927: f64, t12131: f64, t15618: f64, t15691: f64, t15717: f64, t15957: f64, t16025: f64, t16190: f64, t19980: f64, t3117: f64, t3136: f64, t42316: f64, t42804: f64, t43291: f64, t4786: f64, t4887: f64, t12047: f64, t16138: f64, t16158: f64, t3106: f64, t12003: f64, t1659: f64, t11648: f64, t4879: f64, t1042: f64, t1068: f64, t11286: f64, t11679: f64, t11705: f64, t11983: f64, t11994: f64, t15696: f64, t15697: f64, t16140: f64, t3075: f64, t3096: f64, t3127: f64, t4186: f64, t42155: f64, t4834: f64, t4872: f64, t1063: f64, t15790: f64, t3172: f64, t11223: f64, t16088: f64, t380: f64, t1041: f64, t16185: f64, t11202: f64, t11637: f64, t11933: f64, t15139: f64, t16078: f64, t16091: f64, t16095: f64, t42571: f64, t43017: f64, t43019: f64, t43057: f64, t4573: f64, t4875: f64, t42415: f64, t4890: f64, t1062: f64, t42261: f64, t11913: f64, t15719: f64, t15850: f64, t15975: f64, t16049: f64, t3101: f64, t3299: f64, t3317: f64, t43029: f64, t43032: f64, t43035: f64, t43038: f64, t43121: f64, t4896: f64, t4902: f64, t4912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54656, t54658, t54667, t54672, t54678, t54680) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2995(t11656, t15769, t16199, t372, t127, t15700, t15702, t4806, t16208, t15666, t3211, t15656, t3215);
        let t54684 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2996(t11806, t15689, t15700, t15701, t15745, t16222, t19878, t3220, t42900, t53729, t53735, t54263, t54656, t54658, t54667, t54672, t54678, t54680);
        let (t54687, t54693, t54695, t54696, t54699, t54704) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2997(t1025, t1663, t2434, t371, t127, t15649, t225, t53166, t366, t1053, t15655, t15666, t3224);
        let t54712 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2998(t11991, t4817, t1028, t11792, t15651, t1665, t3208, t3211, t42279, t42902, t42907, t4854, t54687, t54693, t54696, t54699, t54704);
        let t54735 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2999(t15731, t3169, t11281, t11774, t11883, t1469, t15707, t15725, t15804, t16149, t3241, t372, t42926, t42929, t42932, t42947, t42962, t4801, t4916, t54398);
        let t54770 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3000(t15816, t3168, t10326, t10356, t1047, t11144, t11675, t15599, t15601, t15622, t1592, t3091, t3092, t3094, t3095, t357, t42410, t42610, t42965, t42996, t43003, t43297, t4583, t4781, t54026);
        let t54806 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3001(t11710, t15591, t3091, t16060, t3241, t1011, t140, t16122, t12078, t53740, t11661, t11684, t11696, t11883, t11927, t12131, t15618, t15689, t15691, t15700, t15717, t15957, t16025, t16190, t19980, t3117, t3136, t42316, t42804, t43291, t4786, t4887);
        let t54843 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3002(t12047, t53740, t16138, t372, t16158, t3106, t12003, t1659, t11648, t4879, t1042, t1068, t11286, t11679, t11705, t11774, t11983, t11994, t12131, t15689, t15691, t15696, t15697, t15707, t16140, t19980, t3075, t3095, t3096, t3127, t4186, t42155, t42804, t4834, t4872);
        let t54880 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3003(t1063, t15790, t3172, t11223, t16088, t380, t1041, t16185, t11202, t11637, t11774, t11933, t15139, t16078, t16091, t16095, t19980, t3092, t3117, t357, t42410, t42571, t43017, t43019, t43057, t43291, t4573, t4781, t4875);
        let t54904 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3004(t42415, t4890, t1062, t42261, t11913, t15719, t15850, t15975, t16049, t3101, t3299, t3317, t43029, t43032, t43035, t43038, t43121, t4834, t4896, t4902, t4912);
    (t54684, t54695, t54712, t54735, t54770, t54806, t54843, t54880, t54904)
}
