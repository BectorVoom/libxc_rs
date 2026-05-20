//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta421 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1590;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1591;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1592;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1593;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta421<F: Float>(t3153: F, t4866: F, t4894: F, t3117: F, t3133: F, t3154: F, t4893: F, t13396: F, t4801: F, t1042: F, t11922: F, t4911: F, t3115: F, t15158: F, t4915: F, t1469: F, t3075: F, t4872: F, t1011: F, t1063: F, t11753: F, t11756: F, t11763: F, t11866: F, t3127: F, t3241: F, t4892: F, t4907: F, t4916: F, t4920: F, t1032: F, t4743: F, t1040: F, t1647: F, t3140: F, t3149: F, t11921: F, t247: F, t4757: F, t4837: F, t1659: F, t3105: F, t4806: F, t1651: F, t3116: F, t1066: F, t15193: F, t1062: F, t4797: F, t1047: F, t1068: F, t11991: F, t1675: F, t3136: F, t3157: F, t3177: F, t3188: F, t4831: F, t4834: F, t4879: F, t3230: F, t1660: F, t3201: F, t1058: F, t4798: F, t1053: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11890: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15780, t15782, t15787, t15791, t15794) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1590::<F>(t3153, t4866, t4894, t3117, t3133, t3154, t4893, t13396, t4801, t1042, t11922, t4911);
        let t15814 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1591::<F>(t15794, t3115, t15158, t4915, t1469, t3075, t4872, t1042, t1011, t1063, t11753, t11756, t11763, t11866, t15782, t15787, t15791, t3127, t3241, t4892, t4907, t4916, t4920);
        let (t15817, t15822, t15823, t15829, t15830) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1592::<F>(t1032, t4743, t1040, t1647, t3140, t3149, t11921, t247, t4757, t4837, t1659, t3105);
        let (t15837, t15855) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1593::<F>(t13396, t4806, t1042, t1651, t3075, t247, t3116, t1066, t15193, t1062, t4797, t1047, t1063, t1068, t11991, t15817, t15823, t15829, t15830, t1675, t3136, t3157, t3177, t3188, t4831, t4834, t4837, t4879);
        let (t15859, t15862, t15865, t15866, t15885) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1594::<F>(t1659, t3230, t1660, t3201, t1058, t4798, t1053, t4797, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11890, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15780, t15814, t15822, t15837, t15855, t15859, t15862, t15865, t15866, t15885)
}
