//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1590;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1591;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1592;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1593;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta421(t3153: f64, t4866: f64, t4894: f64, t3117: f64, t3133: f64, t3154: f64, t4893: f64, t13396: f64, t4801: f64, t1042: f64, t11922: f64, t4911: f64, t3115: f64, t15158: f64, t4915: f64, t1469: f64, t3075: f64, t4872: f64, t1011: f64, t1063: f64, t11753: f64, t11756: f64, t11763: f64, t11866: f64, t3127: f64, t3241: f64, t4892: f64, t4907: f64, t4916: f64, t4920: f64, t1032: f64, t4743: f64, t1040: f64, t1647: f64, t3140: f64, t3149: f64, t11921: f64, t247: f64, t4757: f64, t4837: f64, t1659: f64, t3105: f64, t4806: f64, t1651: f64, t3116: f64, t1066: f64, t15193: f64, t1062: f64, t4797: f64, t1047: f64, t1068: f64, t11991: f64, t1675: f64, t3136: f64, t3157: f64, t3177: f64, t3188: f64, t4831: f64, t4834: f64, t4879: f64, t3230: f64, t1660: f64, t3201: f64, t1058: f64, t4798: f64, t1053: f64, t15127: f64, t15125: f64, t15191: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11890: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15780, t15782, t15787, t15791, t15794) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1590(t3153, t4866, t4894, t3117, t3133, t3154, t4893, t13396, t4801, t1042, t11922, t4911);
        let t15814 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1591(t15794, t3115, t15158, t4915, t1469, t3075, t4872, t1042, t1011, t1063, t11753, t11756, t11763, t11866, t15782, t15787, t15791, t3127, t3241, t4892, t4907, t4916, t4920);
        let (t15817, t15822, t15823, t15829, t15830) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1592(t1032, t4743, t1040, t1647, t3140, t3149, t11921, t247, t4757, t4837, t1659, t3105);
        let (t15837, t15855) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1593(t13396, t4806, t1042, t1651, t3075, t247, t3116, t1066, t15193, t1062, t4797, t1047, t1063, t1068, t11991, t15817, t15823, t15829, t15830, t1675, t3136, t3157, t3177, t3188, t4831, t4834, t4837, t4879);
        let (t15859, t15862, t15865, t15866, t15885) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1594(t1659, t3230, t1660, t3201, t1058, t4798, t1053, t4797, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11890, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15780, t15814, t15822, t15837, t15855, t15859, t15862, t15865, t15866, t15885)
}
