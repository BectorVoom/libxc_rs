//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta923 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2984;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2985;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2986;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta923(t19658: f64, t4879: f64, t4772: f64, t6258: f64, t23633: f64, t4786: f64, t23842: f64, t1011: f64, t1042: f64, t1063: f64, t11774: f64, t11994: f64, t15700: f64, t15701: f64, t15725: f64, t16190: f64, t16222: f64, t1651: f64, t1675: f64, t18281: f64, t19663: f64, t23859: f64, t23863: f64, t23966: f64, t247: f64, t3116: f64, t3127: f64, t4834: f64, t4837: f64, t4872: f64, t4915: f64, t4919: f64, t53320: f64, t53322: f64, t53332: f64, t53473: f64, t5825: f64, t6302: f64, t65689: f64, t67269: f64, t77513: f64, t77579: f64, t77584: f64, t78785: f64, t23862: f64, t3172: f64, t1041: f64, t23822: f64, t4866: f64, t6244: f64, t11710: f64, t23920: f64, t3091: f64, t1058: f64, t23961: f64, t1045: f64, t1053: f64, t11672: f64, t11927: f64, t15618: f64, t19572: f64, t19620: f64, t19716: f64, t19731: f64, t19738: f64, t19741: f64, t19873: f64, t20066: f64, t20070: f64, t23823: f64, t23837: f64, t23921: f64, t23960: f64, t24009: f64, t3117: f64, t3169: f64, t375: f64, t42765: f64, t43291: f64, t4899: f64, t53432: f64, t53437: f64, t53926: f64, t6263: f64, t65738: f64, t11859: f64, t11922: f64, t24008: f64, t23820: f64, t73: f64, t23934: f64, t999: f64, t19477: f64, t1043: f64, t11631: f64, t11875: f64, t15906: f64, t16081: f64, t1668: f64, t19634: f64, t19639: f64, t19682: f64, t19688: f64, t23929: f64, t23997: f64, t3115: f64, t42274: f64, t42643: f64, t4910: f64, t53543: f64, t54916: f64, t6273: f64, t65144: f64, t65801: f64, t65803: f64, t65807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79084, t79097, t79101, t79105) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2984(t19658, t4879, t4772, t6258, t23633, t4786, t23842, t1011, t1042, t1063, t11774, t11994, t15700, t15701, t15725, t16190, t16222, t1651, t1675, t18281, t19663, t23859, t23863, t23966, t247, t3116, t3127, t4834, t4837, t4872, t4915, t4919, t53320, t53322, t53332, t53473, t5825, t6302, t65689, t67269, t77513, t77579, t77584, t78785);
        let (t79107, t79112, t79116, t79139, t79141) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2985(t23862, t3172, t4837, t1041, t23822, t4866, t6244, t11710, t23920, t3091, t1058, t23961);
        let t79151 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2986(t1045, t1053, t11672, t11927, t15618, t19572, t19620, t19716, t19731, t19738, t19741, t19873, t20066, t20070, t23823, t23837, t23921, t23960, t24009, t3117, t3169, t375, t42765, t43291, t4899, t53432, t53437, t53926, t6263, t65738, t79107, t79112, t79116, t79139, t79141);
        let (t79159, t79175, t79180, t79206) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2987(t11859, t11922, t24008, t23820, t73, t23934, t999, t1651, t19477, t1043, t1045, t11631, t11875, t15700, t15906, t16081, t16222, t1668, t19572, t19634, t19639, t19682, t19688, t23929, t23997, t24009, t3115, t3117, t42274, t42643, t4834, t4910, t53543, t54916, t6273, t65144, t65801, t65803, t65807, t79101);
    (t79084, t79097, t79105, t79116, t79151, t79159, t79175, t79180, t79206)
}
