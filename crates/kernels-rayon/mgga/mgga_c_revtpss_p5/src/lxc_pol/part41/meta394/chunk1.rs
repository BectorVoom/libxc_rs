//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1333/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1333(t1196: f64, t20397: f64, t300: f64, t6513: f64, t1198: f64, t16784: f64, t1765: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> (f64, f64, f64, f64) {
    let t20399 = 0.34631718211362927518e2_f64 * t1196 * t20397;
    let t20400 = t300 * t6513;
    let t20402 = 0.5848223622634646207e0_f64 * t20400 * t1198;
    let t20404 = 0.11696447245269292414e1_f64 * t16784 * t1765;
    let t20425 = 0.66437037037037037037e-1_f64 * t20283 - 0.19931111111111111111e0_f64 * t20285 - 0.99655555555555555557e-1_f64 * t20287 + 0.29896666666666666667e0_f64 * t20290 + 0.33218518518518518518e0_f64 * t20295 - 0.11958666666666666667e1_f64 * t20300 - 0.39862222222222222222e0_f64 * t20304 + 0.17938e1_f64 * t20308 + 0.11958666666666666667e1_f64 * t20312 - 0.19931111111111111111e0_f64 * t20315 + 0.59793333333333333334e0_f64 * t20320;
    (t20399, t20402, t20404, t20425)
}
