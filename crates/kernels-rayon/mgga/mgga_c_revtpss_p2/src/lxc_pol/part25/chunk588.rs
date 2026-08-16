//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 588/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk588(t3140: f64, t3143: f64, t342: f64, t3151: f64, t378: f64, t335: f64, t368: f64) -> (f64, f64, f64, f64) {
    let t3298 = t3140 * t3143;
    let t3299 = t342 * t3298;
    let t3300 = t378 * t3151;
    let t3302 = 1.0_f64 / t368 / t335;
    (t3298, t3299, t3300, t3302)
}
