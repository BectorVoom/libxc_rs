//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta923 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2984;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2985;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2986;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta923<F: Float>(t19658: F, t4879: F, t4772: F, t6258: F, t23633: F, t4786: F, t23842: F, t1011: F, t1042: F, t1063: F, t11774: F, t11994: F, t15700: F, t15701: F, t15725: F, t16190: F, t16222: F, t1651: F, t1675: F, t18281: F, t19663: F, t23859: F, t23863: F, t23966: F, t247: F, t3116: F, t3127: F, t4834: F, t4837: F, t4872: F, t4915: F, t4919: F, t53320: F, t53322: F, t53332: F, t53473: F, t5825: F, t6302: F, t65689: F, t67269: F, t77513: F, t77579: F, t77584: F, t78785: F, t23862: F, t3172: F, t1041: F, t23822: F, t4866: F, t6244: F, t11710: F, t23920: F, t3091: F, t1058: F, t23961: F, t1045: F, t1053: F, t11672: F, t11927: F, t15618: F, t19572: F, t19620: F, t19716: F, t19731: F, t19738: F, t19741: F, t19873: F, t20066: F, t20070: F, t23823: F, t23837: F, t23921: F, t23960: F, t24009: F, t3117: F, t3169: F, t375: F, t42765: F, t43291: F, t4899: F, t53432: F, t53437: F, t53926: F, t6263: F, t65738: F, t11859: F, t11922: F, t24008: F, t23820: F, t73: F, t23934: F, t999: F, t19477: F, t1043: F, t11631: F, t11875: F, t15906: F, t16081: F, t1668: F, t19634: F, t19639: F, t19682: F, t19688: F, t23929: F, t23997: F, t3115: F, t42274: F, t42643: F, t4910: F, t53543: F, t54916: F, t6273: F, t65144: F, t65801: F, t65803: F, t65807: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t79084, t79097, t79101, t79105) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2984::<F>(t19658, t4879, t4772, t6258, t23633, t4786, t23842, t1011, t1042, t1063, t11774, t11994, t15700, t15701, t15725, t16190, t16222, t1651, t1675, t18281, t19663, t23859, t23863, t23966, t247, t3116, t3127, t4834, t4837, t4872, t4915, t4919, t53320, t53322, t53332, t53473, t5825, t6302, t65689, t67269, t77513, t77579, t77584, t78785);
        let (t79107, t79112, t79116, t79139, t79141) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2985::<F>(t23862, t3172, t4837, t1041, t23822, t4866, t6244, t11710, t23920, t3091, t1058, t23961);
        let t79151 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2986::<F>(t1045, t1053, t11672, t11927, t15618, t19572, t19620, t19716, t19731, t19738, t19741, t19873, t20066, t20070, t23823, t23837, t23921, t23960, t24009, t3117, t3169, t375, t42765, t43291, t4899, t53432, t53437, t53926, t6263, t65738, t79107, t79112, t79116, t79139, t79141);
        let (t79159, t79175, t79180, t79206) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2987::<F>(t11859, t11922, t24008, t23820, t73, t23934, t999, t1651, t19477, t1043, t1045, t11631, t11875, t15700, t15906, t16081, t16222, t1668, t19572, t19634, t19639, t19682, t19688, t23929, t23997, t24009, t3115, t3117, t42274, t42643, t4834, t4910, t53543, t54916, t6273, t65144, t65801, t65803, t65807, t79101);
    (t79084, t79097, t79105, t79116, t79151, t79159, t79175, t79180, t79206)
}
