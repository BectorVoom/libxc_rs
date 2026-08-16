//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk685;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta129(t3223: f64, t366: f64, t1054: f64, t1058: f64, t1014: f64, t2857: f64, t1010: f64, t614: f64, t1016: f64, t140: f64, t1011: f64, t271: f64, t905: f64, t2852: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3224, t3234, t3236, t3241, t3245, t3252) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk685(t3223, t366, t1054, t1058, t1014, t2857, t1010, t614, t1016, t140, t1011, t271, t905);
        let (t3253, t3264, t3268, t3269) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk686(t2852, t3252, t1071, t342, t1077, t384, t225);
    (t3224, t3234, t3236, t3241, t3245, t3252, t3253, t3264, t3268, t3269)
}
