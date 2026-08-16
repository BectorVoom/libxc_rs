//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 716/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk716(t1943: f64, t4431: f64, t4714: f64, t72: f64, t1526: f64, t1527: f64, t16631: f64, t16649: f64, t20507: f64, t20514: f64, t3088: f64, t342: f64, t343: f64, t4650: f64, t4656: f64, t4720: f64, t8759: f64) -> (f64, f64, f64) {
    let t20518 = t1943 * t4431;
    let t20522 = t72 * t4714;
    let t20526 = t4650 + t4720 + t8759 - t16631 / 18.0_f64 - t16649 / 6.0_f64 - t1526 * t3088 * t20507 / 9.0_f64 - t1526 * t1527 * t4656 / 6.0_f64 + t1526 * t1527 * t20514 / 6.0_f64 - t1526 * t1527 * t20518 / 12.0_f64 - t342 * t343 * t20522 / 4.0_f64;
    (t20518, t20522, t20526)
}
