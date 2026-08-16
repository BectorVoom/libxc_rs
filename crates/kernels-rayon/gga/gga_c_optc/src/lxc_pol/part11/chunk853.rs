//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 853/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk853(t16416: f64, t16438: f64, t16464: f64, t16471: f64, t16474: f64, t16479: f64, t16483: f64, t16487: f64, t16490: f64, t16493: f64, t16496: f64, t2113: f64, t2159: f64, t673: f64, t686: f64, t695: f64, t6993: f64, t7002: f64, t705: f64) -> f64 {
    let t16499 = -0.20863587575493018851e1_f64 * t686 * t16464 - 0.30228422675018518372e-1_f64 * t705 * t16438 - 0.90685268025055555116e0_f64 * t705 * t16416 + 0.13602790203758333267e0_f64 * t2159 * t16471 - 0.18137053605011111023e0_f64 * t6993 * t16474 - 0.52158968938732547127e0_f64 * t7002 * t16479 + 0.52158968938732547127e0_f64 * t2113 * t16483 - 0.86931614897887578546e-1_f64 * t673 * t16487 + 0.45342634012527777558e0_f64 * t705 * t16490 + 0.15647690681619764138e1_f64 * t686 * t16493 - 0.15114211337509259186e-1_f64 * t695 * t16496;
    t16499
}
