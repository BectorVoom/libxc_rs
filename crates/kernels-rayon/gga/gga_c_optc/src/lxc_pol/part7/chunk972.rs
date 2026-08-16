//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 972/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk972(t275: f64, t9337: f64, t176: f64, t498: f64, t8560: f64, t8564: f64, t8574: f64, t8703: f64, t8705: f64, t8707: f64, t8745: f64, t8747: f64, t8753: f64, t8898: f64, t9266: f64, sigma2: f64) -> (f64, f64) {
    let t9338 = t9337 * t275;
    let t9340 = t176 * t9338 * sigma2;
    let t9343 = -t8560 + t8564 + t8574 - t8703 - t8705 - t8707 + t9266 / 2.0_f64 + t9340 * t498 / 2.0_f64 - t8745 + t8747 + t8753 - t8898;
    (t9340, t9343)
}
