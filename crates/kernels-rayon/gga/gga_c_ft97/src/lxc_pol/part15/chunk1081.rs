//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1081/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1081(t20972: f64, t3578: f64, t1526: f64, t1527: f64, t1943: f64, t20044: f64, t20527: f64, t20529: f64, t20655: f64, t20678: f64, t342: f64, t343: f64, t41328: f64, t64663: f64, t64677: f64, t72: f64, t78650: f64, t78653: f64, t78700: f64) -> (f64, f64) {
    let t87220 = t3578 * t20972;
    let t87252 = t78650 / 6.0_f64 + t64677 / 6.0_f64 - t78653 / 12.0_f64 + t20527 + t20678 - t41328 + 2.0_f64 * t20529 - t342 * t343 * t72 * t20655 / 4.0_f64 - t1526 * t1527 * t1943 * t20044 / 12.0_f64 + t64663 / 18.0_f64 - t78700 / 4.0_f64;
    (t87220, t87252)
}
