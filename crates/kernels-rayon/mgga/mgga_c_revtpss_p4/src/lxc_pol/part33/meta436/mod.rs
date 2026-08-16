//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1572;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1573;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1574;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1575;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1576;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta436(t19855: f64, t341: f64, t225: f64, t366: f64, t15696: f64, t4782: f64, t4787: f64, t1058: f64, t6318: f64, t1053: f64, t6317: f64, t4786: f64, t6096: f64, t3092: f64, t1062: f64, t15670: f64, t247: f64, t3109: f64, t1063: f64, t11672: f64, t11774: f64, t15796: f64, t15829: f64, t3091: f64, t375: f64, t4839: f64, t6268: f64, t19691: f64, t4801: f64, t1042: f64, t140: f64, t6284: f64, t1011: f64, t6288: f64, t6292: f64, t1015: f64, t18281: f64, t1012: f64, t3172: f64, t6262: f64, t3127: f64, t11881: f64, t15986: f64, t15990: f64, t15996: f64, t16037: f64, t3241: f64, t6289: f64, t6293: f64, t15935: f64, t19661: f64, t19666: f64, t1592: f64, t16138: f64, t19399: f64, t3116: f64, t18942: f64, t4915: f64, t11656: f64, t11994: f64, t11999: f64, t16057: f64, t16062: f64, t16064: f64, t4837: f64, t6263: f64, t6312: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t19856, t19858, t19861, t19864, t19867, t19869, t19872) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1572(t19855, t341, t225, t366, t15696, t4782, t4787, t1058, t6318, t1053, t6317, t4786, t6096);
        let t19885 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1573(t19872, t3092, t1062, t15670, t247, t3109, t6096, t1063, t11672, t11774, t15796, t15829, t19858, t19861, t19864, t19867, t19869, t3091, t375, t4839, t6268);
        let (t19895, t19901, t19908, t19913, t19917, t19920) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1574(t19691, t4801, t1042, t140, t6284, t1011, t6288, t6292, t1015, t18281, t1012, t3172, t6262);
        let t19923 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1575(t19920, t3127, t1011, t11881, t15986, t15990, t15996, t16037, t19908, t19913, t19917, t3241, t6289, t6293);
        let (t19930, t19934, t19940, t19944, t19947) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1576(t15935, t19661, t1042, t19666, t4801, t1592, t16138, t19399, t247, t3116, t18942, t4915);
        let t19950 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1577(t1011, t1063, t11656, t11994, t11999, t16057, t16062, t16064, t19930, t19934, t19940, t19944, t19947, t3127, t4837, t6263, t6312);
    (t19856, t19885, t19895, t19901, t19923, t19950)
}
