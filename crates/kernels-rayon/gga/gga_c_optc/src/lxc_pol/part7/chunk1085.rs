//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1085/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1085(t23518: f64, t352: f64, t2663: f64, t275: f64, t2329: f64, t2320: f64, t2347: f64, t287: f64, t745: f64, t355: f64, t357: f64, t362: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23519 = t23518 * t352;
    let t23520 = t2663 * t275;
    let t23523 = t2329 * t2329;
    let t23531 = t2320 * t2347;
    let t23533 = t745 * t287;
    let t23537 = 40.0_f64 / 81.0_f64 * t355 * t357 * t23533 * t362;
    (t23519, t23520, t23523, t23531, t23533, t23537)
}
