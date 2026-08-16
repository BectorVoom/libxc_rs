//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk679;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk680;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta145(t362: f64, t40: f64, t611: f64, t361: f64, t351: f64, t1054: f64, t1058: f64, t1014: f64, t2857: f64, t2251: f64, t1012: f64, t1010: f64, t614: f64, t1016: f64, t140: f64, t1011: f64, t1015: f64, t2258: f64, t271: f64, t905: f64, t2852: f64, t1017: f64, t1025: f64, t1028: f64, t1068: f64, t3188: f64, t3191: f64, t3194: f64, t3197: f64, t3203: f64, t3205: f64, t3208: f64, t3211: f64, t3216: f64, t3220: f64, t3224: f64, t375: f64, t3187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3229, t3230, t3231, t3234, t3237, t3238, t3241) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk679(t362, t40, t611, t361, t351, t1054, t1058, t1014, t2857, t2251, t1012, t1010, t614);
        let (t3244, t3245, t3247, t3252, t3254, t3258) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk680(t1016, t140, t1011, t1015, t2258, t1012, t271, t905, t2852, t2251, t1017, t1025, t1028, t1068, t3188, t3191, t3194, t3197, t3203, t3205, t3208, t3211, t3216, t3220, t3224, t3231, t3234, t3238, t3241, t375);
        let t3259 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk681(t3187, t3258);
    (t3229, t3230, t3231, t3234, t3237, t3241, t3244, t3245, t3247, t3252, t3254, t3259)
}
