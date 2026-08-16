//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1652/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1652(t11349: f64, t11378: f64, t935: f64, t915: f64, t2922: f64, t913: f64, t275: f64) -> (f64, f64, f64, f64, f64) {
    let t11379 = t11349 + t11378;
    let t11380 = t11379 * t935;
    let t11382 = 1.0_f64 * t915 * t11380;
    let t11384 = 1.0_f64 / t2922 / t913;
    let t11385 = t275 * t11384;
    (t11379, t11380, t11382, t11384, t11385)
}
