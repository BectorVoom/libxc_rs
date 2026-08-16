//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 125/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk125(t218: f64, t219: f64, t344: f64, t336: f64, t339: f64, t342: f64) -> (f64, f64, f64, f64) {
    let t346 = t218 * t219 * t344;
    let t348 = 0.379785e1_f64 * t339 + 0.8969e0_f64 * t336 + 0.204775e0_f64 * t342 + 0.123235e0_f64 * t346;
    let t351 = 1.0_f64 + 0.16081979498692535067e2_f64 / t348;
    let t352 = f64::ln(t351);
    (t346, t348, t351, t352)
}
