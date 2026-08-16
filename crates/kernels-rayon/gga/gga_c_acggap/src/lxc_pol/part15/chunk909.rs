//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 909/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk909(t2037: f64, t31035: f64, t377: f64, t7684: f64, t409: f64, t1: f64, t2065: f64, t2066: f64, t1160: f64) -> (f64, f64, f64, f64, f64) {
    let t31036 = t31035 * t2037;
    let t31038 = t377 * t7684;
    let t31039 = t31038 * t409;
    let t31056 = t2065 * t2066 * t1;
    let t31057 = t1160 * t31056;
    (t31036, t31038, t31039, t31056, t31057)
}
