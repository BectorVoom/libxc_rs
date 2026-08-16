//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1030/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1030(t195: f64, t6: f64, t8285: f64, t92: f64, t93: f64, t2663: f64, t275: f64, t287: f64, t745: f64, t355: f64, t357: f64, t362: f64) -> (f64, f64, f64, f64, f64) {
    let t23471 = t6 * t195;
    let t23518 = 1.0_f64 / t8285 / t92 * t93;
    let t23520 = t2663 * t275;
    let t23533 = t745 * t287;
    let t23537 = 40.0_f64 / 81.0_f64 * t355 * t357 * t23533 * t362;
    (t23471, t23518, t23520, t23533, t23537)
}
