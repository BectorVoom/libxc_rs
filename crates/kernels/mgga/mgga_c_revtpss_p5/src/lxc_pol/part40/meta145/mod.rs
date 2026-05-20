//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk679;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk680;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta145<F: Float>(t362: F, t40: F, t611: F, t361: F, t351: F, t1054: F, t1058: F, t1014: F, t2857: F, t2251: F, t1012: F, t1010: F, t614: F, t1016: F, t140: F, t1011: F, t1015: F, t2258: F, t271: F, t905: F, t2852: F, t1017: F, t1025: F, t1028: F, t1068: F, t3188: F, t3191: F, t3194: F, t3197: F, t3203: F, t3205: F, t3208: F, t3211: F, t3216: F, t3220: F, t3224: F, t375: F, t3187: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3229, t3230, t3231, t3234, t3237, t3238, t3241) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk679::<F>(t362, t40, t611, t361, t351, t1054, t1058, t1014, t2857, t2251, t1012, t1010, t614);
        let (t3244, t3245, t3247, t3252, t3254, t3258) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk680::<F>(t1016, t140, t1011, t1015, t2258, t1012, t271, t905, t2852, t2251, t1017, t1025, t1028, t1068, t3188, t3191, t3194, t3197, t3203, t3205, t3208, t3211, t3216, t3220, t3224, t3231, t3234, t3238, t3241, t375);
        let t3259 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk681::<F>(t3187, t3258);
    (t3229, t3230, t3231, t3234, t3237, t3241, t3244, t3245, t3247, t3252, t3254, t3259)
}
