//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 932/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk932(t31253: f64, t409: f64, t7712: f64, t932: f64, t2029: f64, t7599: f64, t2032: f64, t2059: f64, t2062: f64, t167: f64, t7309: f64, t7483: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31254 = t31253 * t409;
    let t31256 = t7712 * t932;
    let t31258 = t7599 * t2029;
    let t31259 = t31258 * t2032;
    let t31261 = t7599 * t2059;
    let t31262 = t31261 * t2062;
    let t31276 = t7309 * t167;
    let t31277 = t31276 * t7483;
    (t31254, t31256, t31258, t31259, t31261, t31262, t31276, t31277)
}
