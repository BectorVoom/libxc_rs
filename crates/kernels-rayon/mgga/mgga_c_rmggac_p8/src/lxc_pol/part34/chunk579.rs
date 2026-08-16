//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 579/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk579(t2323: f64, t3056: f64, t3057: f64, t2338: f64, t668: f64, t638: f64, t639: f64, t2405: f64, t640: f64, t2046: f64, t2339: f64, t3047: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15033 = t3056 * t3057 * t2323;
    let t15035 = t2338 * t668;
    let t15037 = t638 * t639 * t15035;
    let t15039 = t640 * t2405;
    let t15041 = t638 * t639 * t15039;
    let t15044 = t2046 * t3047 * t2339;
    (t15033, t15035, t15037, t15039, t15041, t15044)
}
