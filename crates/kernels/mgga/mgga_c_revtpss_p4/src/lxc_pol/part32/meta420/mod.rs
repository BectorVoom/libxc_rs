//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1466;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1467;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1468;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1469;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1470;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1471;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta420<F: Float>(t19456: F, t247: F, t3116: F, t3172: F, t6311: F, t3161: F, t1043: F, t6244: F, t1045: F, t3117: F, t1668: F, t4772: F, t11866: F, t11927: F, t15716: F, t15771: F, t15774: F, t15776: F, t15817: F, t1671: F, t3115: F, t4831: F, t4834: F, t4869: F, t4879: F, t6273: F, t11134: F, t11890: F, t15189: F, t15874: F, t15875: F, t15876: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F, t341: F, t225: F, t366: F, t15696: F, t4782: F, t4787: F, t1058: F, t6318: F, t1053: F, t6317: F, t4786: F, t6096: F, t3092: F, t1062: F, t15670: F, t3109: F, t1063: F, t11672: F, t11774: F, t15796: F, t15829: F, t3091: F, t375: F, t4839: F, t6268: F, t19691: F, t4801: F, t1042: F, t140: F, t6284: F, t1011: F, t6288: F, t6292: F, t1015: F, t18281: F, t1012: F, t6262: F, t3127: F, t11881: F, t15986: F, t15990: F, t15996: F, t16037: F, t3241: F, t6289: F, t6293: F) -> (F, F, F, F, F, F, F, F) {
        let (t19819, t19827, t19829, t19831, t19836) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1466::<F>(t19456, t247, t3116, t3172, t6311, t3161, t1043, t6244, t1045, t3117, t1668, t4772);
        let t19841 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1467::<F>(t1045, t19836, t3117, t11866, t11927, t15716, t15771, t15774, t15776, t15817, t1671, t19819, t19827, t19831, t3115, t4831, t4834, t4869, t4879, t6273);
        let t19855 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1468::<F>(t11134, t11890, t15189, t15874, t15875, t15876, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19856, t19858, t19861, t19864, t19867, t19869, t19872) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1469::<F>(t19855, t341, t225, t366, t15696, t4782, t4787, t1058, t6318, t1053, t6317, t4786, t6096);
        let t19885 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1470::<F>(t19872, t3092, t1062, t15670, t247, t3109, t6096, t1063, t11672, t11774, t15796, t15829, t19858, t19861, t19864, t19867, t19869, t3091, t375, t4839, t6268);
        let (t19895, t19901, t19908, t19913, t19917, t19920) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1471::<F>(t19691, t4801, t1042, t140, t6284, t1011, t6288, t6292, t1015, t18281, t1012, t3172, t6262);
        let t19923 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1472::<F>(t19920, t3127, t1011, t11881, t15986, t15990, t15996, t16037, t19908, t19913, t19917, t3241, t6289, t6293);
    (t19829, t19836, t19841, t19856, t19885, t19895, t19901, t19923)
}
