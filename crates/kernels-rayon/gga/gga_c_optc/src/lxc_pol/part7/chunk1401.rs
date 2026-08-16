//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1401/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1401(t26280: f64, t26284: f64, t26293: f64, t26296: f64, t26304: f64, t26311: f64, t26319: f64, t26324: f64, t26388: f64, t26394: f64, t26396: f64, t26278: f64, t26289: f64, t26300: f64, t26306: f64, t26309: f64, t26406: f64, t26409: f64, t26412: f64, t26415: f64, t26419: f64, t26443: f64, t26446: f64) -> (f64, f64) {
    let t27901 = -0.1642e-2_f64 * t26388 + 0.15510666666666666667e2_f64 * t26319 - 0.5170222222222222222e1_f64 * t26324 - 0.19704e-1_f64 * t26394 + 0.3284e-2_f64 * t26396 + 0.15510666666666666667e2_f64 * t26280 - 0.46531999999999999999e2_f64 * t26284 - 0.38776666666666666665e1_f64 * t26293 + 0.46532e2_f64 * t26296 + 0.11633e2_f64 * t26304 - 0.10340444444444444444e2_f64 * t26311;
    let t27914 = 0.19704e-1_f64 * t26406 - 0.14778e-1_f64 * t26409 - 0.12315e-2_f64 * t26412 + 0.29556e-1_f64 * t26415 + 0.7389e-2_f64 * t26419 - 0.12925555555555555555e2_f64 * t26278 + 0.46531999999999999998e2_f64 * t26289 - 0.69798e2_f64 * t26300 - 0.15510666666666666667e2_f64 * t26306 + 0.5170222222222222222e1_f64 * t26309 + 0.14778e-1_f64 * t26443 - 0.44334e-1_f64 * t26446;
    (t27901, t27914)
}
