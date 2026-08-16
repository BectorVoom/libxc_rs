//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 521/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk521(t3228: f64, t425: f64, t431: f64, t438: f64, t377: f64, t996: f64) -> (f64, f64, f64, f64) {
    let t3229 = t3228 * t425;
    let t3231 = t3228 * t431;
    let t3233 = t3228 * t438;
    let t3237 = t377 * t996;
    (t3229, t3231, t3233, t3237)
}
