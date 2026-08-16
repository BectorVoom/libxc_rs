//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 129/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk129(t413: f64, t8: f64, t6: f64, t93: f64, t408: f64, t411: f64, t414: f64, t88: f64, t392: f64, t402: f64, t405: f64, t70: f64, t73: f64, t99: f64) -> (f64, f64, f64, f64, f64) {
    let t417 = 1.0_f64 / t8 / t413;
    let t418 = t6 * t417;
    let t419 = t93 * t418;
    let t421 = 0.59778596625315888114e-2_f64 * t88 - 0.17565e-2_f64 * t408 + 0.39625e-3_f64 * t411 - 0.1294884726949076719e-4_f64 * t414 + 0.1260328125e-5_f64 * t419;
    let t423 = -0.11713266981940447749e-2_f64 * t88 * t70 - 0.23426533963880895498e-2_f64 * t392 * t402 - t405 * t99 - t73 * t421;
    (t417, t418, t419, t421, t423)
}
