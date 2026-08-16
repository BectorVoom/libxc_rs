//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1331/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1331(t26278: f64, t26289: f64, t26300: f64, t26306: f64, t26309: f64, t26419: f64, t26425: f64, t26428: f64, t26430: f64, t26433: f64, t26435: f64, t26443: f64, t26446: f64, t26448: f64) -> f64 {
    let t26554 = 0.49671e0_f64 * t26419 - 0.485484375e1_f64 * t26425 - 0.247573125e0_f64 * t26428 + 0.3300975e0_f64 * t26430 + 0.11651625e2_f64 * t26433 - 0.51785e1_f64 * t26435 - 0.20128333333333333334e1_f64 * t26278 + 0.72462e1_f64 * t26289 - 0.108693e2_f64 * t26300 - 0.24154e1_f64 * t26306 + 0.80513333333333333333e0_f64 * t26309 + 0.99342e0_f64 * t26443 - 0.298026e1_f64 * t26446 + 0.16504875e0_f64 * t26448;
    t26554
}
