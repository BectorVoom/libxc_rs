//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2427/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2427(t273: f64, t270: f64, t276: f64, t39484: f64, t11318: f64, t698: f64, t9303: f64, t931: f64, t11571: f64, t300: f64, t2922: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41382 = f64::powf(t273, -0.25e1_f64);
    let t41401 = 1.0_f64 / t276 / t39484 / t270 / 96.0_f64;
    let t41406 = t698 * t11318;
    let t41441 = t9303 * t931;
    let t41491 = t300 * t11571;
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    (t41382, t41401, t41406, t41441, t41491, t41499)
}
