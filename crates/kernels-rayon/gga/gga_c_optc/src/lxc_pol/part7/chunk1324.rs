//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1324/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1324(t1045: f64, t26346: f64, t26278: f64, t26289: f64, t26300: f64, t26306: f64, t26309: f64, t26419: f64, t26425: f64, t26428: f64, t26430: f64, t26433: f64, t26435: f64, t26443: f64, t26446: f64) -> (f64, f64) {
    let t26448 = t1045 * t26346;
    let t26450 = 0.49293999999999999999e0_f64 * t26419 - 0.3560484375e1_f64 * t26425 - 0.46074375e0_f64 * t26428 + 0.614325e0_f64 * t26430 + 0.85451625e1_f64 * t26433 - 0.379785e1_f64 * t26435 - 0.19931111111111111111e1_f64 * t26278 + 0.71752000000000000001e1_f64 * t26289 - 0.107628e2_f64 * t26300 - 0.23917333333333333333e1_f64 * t26306 + 0.79724444444444444444e0_f64 * t26309 + 0.98587999999999999999e0_f64 * t26443 - 0.295764e1_f64 * t26446 + 0.3071625e0_f64 * t26448;
    (t26448, t26450)
}
