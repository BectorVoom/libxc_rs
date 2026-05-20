//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta805 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2928;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2929;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2930;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2931;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2932;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta805<F: Float>(t53251: F, t53272: F, t11223: F, t1678: F, t16163: F, t3169: F, t1041: F, t11262: F, t4868: F, t1058: F, t15859: F, t3201: F, t4794: F, t15866: F, t15888: F, t11656: F, t11961: F, t12004: F, t15811: F, t1659: F, t225: F, t366: F, t375: F, t4803: F, t4808: F, t52977: F, t4798: F, t343: F, t44: F, t816: F, t11821: F, t65: F, t11144: F, t11970: F, t1660: F, t27527: F, t2852: F, t11150: F, t27531: F, t15908: F, t999: F, t1042: F, t1053: F, t11804: F, t15716: F, t15887: F, t15907: F, t1663: F, t247: F, t3116: F, t3117: F, t3230: F, t42967: F, t43105: F, t4788: F, t4797: F, t4837: F, t51959: F, t53192: F, t15817: F, t3173: F, t16158: F, t3188: F, t1063: F, t15193: F, t3109: F, t11233: F, t12026: F, t15707: F, t15791: F, t15830: F, t15834: F, t15952: F, t3106: F, t3177: F, t3184: F, t42391: F, t4825: F, t4834: F, t11710: F, t15600: F, t3091: F, t127: F, t4823: F, t11774: F, t3096: F, t11675: F, t15592: F, t15596: F, t42121: F, t42122: F, t42124: F, t42139: F, t42141: F, t42146: F, t42149: F) -> (F, F, F, F, F, F, F) {
        let (t53273, t53281, t53290, t53294, t53298, t53300) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2928::<F>(t53251, t53272, t11223, t1678, t16163, t3169, t1041, t11262, t4868, t1058, t15859, t3201, t4794);
        let t53310 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2929::<F>(t1058, t15866, t15888, t11656, t11961, t12004, t15811, t1659, t225, t366, t375, t4803, t4808, t52977, t53290, t53294, t53298, t53300);
        let (t53318, t53320, t53322, t53326, t53328) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2930::<F>(t3201, t4798, t343, t44, t816, t11821, t65, t11144, t11970, t1660, t27527, t2852);
        let (t53340, t53351) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2931::<F>(t11150, t27531, t15908, t999, t1042, t1053, t11804, t15716, t15887, t15907, t1663, t247, t3116, t3117, t3230, t375, t42967, t43105, t4788, t4797, t4837, t51959, t53192, t53318, t53320, t53322, t53326, t53328);
        let t53377 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2932::<F>(t15817, t3173, t16158, t3188, t1063, t15193, t247, t3109, t11233, t11656, t12026, t15707, t15791, t15830, t15834, t15952, t3106, t3177, t3184, t42391, t4825, t4834);
        let t53395 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2933::<F>(t11710, t15600, t3091, t127, t4823, t11774, t3096, t11675, t15592, t15596, t42121, t42122, t42124, t42139, t42141, t42146, t42149);
    (t53273, t53281, t53310, t53340, t53351, t53377, t53395)
}
