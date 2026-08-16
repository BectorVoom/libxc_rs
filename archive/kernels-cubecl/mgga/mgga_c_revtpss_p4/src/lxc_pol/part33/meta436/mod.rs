//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta436 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1572;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1573;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1574;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1575;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1576;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta436<F: Float>(t19855: F, t341: F, t225: F, t366: F, t15696: F, t4782: F, t4787: F, t1058: F, t6318: F, t1053: F, t6317: F, t4786: F, t6096: F, t3092: F, t1062: F, t15670: F, t247: F, t3109: F, t1063: F, t11672: F, t11774: F, t15796: F, t15829: F, t3091: F, t375: F, t4839: F, t6268: F, t19691: F, t4801: F, t1042: F, t140: F, t6284: F, t1011: F, t6288: F, t6292: F, t1015: F, t18281: F, t1012: F, t3172: F, t6262: F, t3127: F, t11881: F, t15986: F, t15990: F, t15996: F, t16037: F, t3241: F, t6289: F, t6293: F, t15935: F, t19661: F, t19666: F, t1592: F, t16138: F, t19399: F, t3116: F, t18942: F, t4915: F, t11656: F, t11994: F, t11999: F, t16057: F, t16062: F, t16064: F, t4837: F, t6263: F, t6312: F) -> (F, F, F, F, F, F) {
        let (t19856, t19858, t19861, t19864, t19867, t19869, t19872) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1572::<F>(t19855, t341, t225, t366, t15696, t4782, t4787, t1058, t6318, t1053, t6317, t4786, t6096);
        let t19885 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1573::<F>(t19872, t3092, t1062, t15670, t247, t3109, t6096, t1063, t11672, t11774, t15796, t15829, t19858, t19861, t19864, t19867, t19869, t3091, t375, t4839, t6268);
        let (t19895, t19901, t19908, t19913, t19917, t19920) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1574::<F>(t19691, t4801, t1042, t140, t6284, t1011, t6288, t6292, t1015, t18281, t1012, t3172, t6262);
        let t19923 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1575::<F>(t19920, t3127, t1011, t11881, t15986, t15990, t15996, t16037, t19908, t19913, t19917, t3241, t6289, t6293);
        let (t19930, t19934, t19940, t19944, t19947) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1576::<F>(t15935, t19661, t1042, t19666, t4801, t1592, t16138, t19399, t247, t3116, t18942, t4915);
        let t19950 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1577::<F>(t1011, t1063, t11656, t11994, t11999, t16057, t16062, t16064, t19930, t19934, t19940, t19944, t19947, t3127, t4837, t6263, t6312);
    (t19856, t19885, t19895, t19901, t19923, t19950)
}
