//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta922 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2979;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2980;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2981;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2982;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta922<F: Float>(t1651: F, t19380: F, t1043: F, t23598: F, t23916: F, t3091: F, t43131: F, t1045: F, t11703: F, t15830: F, t15850: F, t16067: F, t16089: F, t16095: F, t19450: F, t19716: F, t19968: F, t20046: F, t247: F, t3092: F, t3115: F, t3116: F, t3117: F, t4578: F, t4757: F, t4831: F, t4834: F, t4837: F, t4894: F, t4907: F, t53294: F, t53669: F, t6092: F, t6323: F, t6331: F, t65454: F, t65456: F, t65459: F, t65462: F, t65471: F, t67551: F, t78524: F, t78616: F, t78812: F, t15618: F, t19785: F, t23820: F, t3153: F, t1668: F, t5825: F, t54397: F, t5819: F, t19620: F, t11774: F, t15584: F, t15689: F, t15700: F, t15701: F, t15707: F, t15758: F, t16222: F, t16226: F, t19634: F, t19639: F, t19641: F, t19702: F, t20075: F, t23931: F, t23934: F, t4808: F, t4892: F, t4899: F, t4900: F, t53300: F, t53318: F, t53326: F, t53800: F, t54471: F, t54570: F, t6268: F, t66565: F, t19920: F, t23891: F, t3127: F, t3172: F, t1063: F, t1066: F, t11250: F, t11632: F, t11927: F, t19677: F, t19930: F, t19934: F, t23976: F, t23992: F, t24007: F, t3106: F, t3188: F, t42621: F, t43044: F, t43105: F, t53619: F, t65488: F, t65493: F, t65507: F, t65510: F, t65527: F, t65538: F, t65553: F, t77501: F, t19697: F, t4820: F, t1011: F, t1042: F, t11656: F, t11859: F, t11875: F, t16012: F, t16208: F, t1671: F, t19649: F, t19792: F, t23892: F, t23997: F, t3095: F, t3155: F, t4583: F, t4866: F, t53692: F, t53944: F, t6263: F, t6271: F, t65342: F, t65567: F, t65570: F, t65581: F, t65585: F, t77564: F, t77568: F, t77573: F, t78496: F, t78790: F, t1032: F, t1040: F, t23959: F, t1047: F, t11860: F, t15906: F, t16199: F, t19722: F, t19748: F, t19971: F, t22671: F, t23830: F, t42121: F, t42690: F, t43207: F, t4872: F, t4910: F, t54316: F, t65589: F, t65596: F, t65598: F, t65610: F, t65618: F, t65627: F, t65630: F, t65637: F, t65650: F, t66431: F, t999: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t78826, t78831, t78857) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2979::<F>(t1651, t19380, t1043, t23598, t23916, t3091, t43131, t1045, t11703, t15830, t15850, t16067, t16089, t16095, t19450, t19716, t19968, t20046, t247, t3092, t3115, t3116, t3117, t4578, t4757, t4831, t4834, t4837, t4894, t4907, t53294, t53669, t6092, t6323, t6331, t65454, t65456, t65459, t65462, t65471, t67551, t78524, t78616, t78812);
        let (t78873, t78884, t78900, t78901, t78909) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2980::<F>(t15618, t19785, t23820, t3153, t1668, t5825, t54397, t5819, t19620, t11774, t15584, t15689, t15700, t15701, t15707, t15758, t16222, t16226, t19634, t19639, t19641, t19702, t19968, t20075, t23931, t23934, t3117, t4808, t4892, t4894, t4899, t4900, t53300, t53318, t53326, t53800, t54471, t54570, t6268, t66565);
        let t78954 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2981::<F>(t15707, t19920, t23891, t3127, t3172, t1043, t1063, t1066, t11250, t11632, t11927, t19620, t19677, t19930, t19934, t23976, t23992, t24007, t247, t3106, t3117, t3188, t42621, t43044, t43105, t4834, t53619, t65488, t65493, t65507, t65510, t65527, t65538, t65553, t77501);
        let t79006 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2982::<F>(t19697, t4820, t1011, t1042, t1063, t11656, t11859, t11875, t11927, t15707, t16012, t16067, t16208, t1671, t19620, t19639, t19649, t19792, t23892, t23992, t23997, t3092, t3095, t3117, t3155, t4583, t4837, t4866, t53692, t53944, t6263, t6271, t65342, t65567, t65570, t65581, t65585, t77564, t77568, t77573, t78496, t78790);
        let t79049 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2983::<F>(t1032, t1040, t23959, t1042, t1047, t1063, t11860, t15906, t16199, t19450, t19722, t19748, t19971, t22671, t23830, t3117, t3127, t42121, t42690, t43105, t43207, t4872, t4910, t54316, t65589, t65596, t65598, t65610, t65618, t65627, t65630, t65637, t65650, t66431, t78496, t78790, t78812, t999);
    (t78826, t78831, t78857, t78873, t78884, t78900, t78901, t78909, t78954, t79006, t79049)
}
