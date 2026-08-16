//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 979/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk979(t21875: f64, t8675: f64, t13598: f64, t1526: f64, t21911: f64, t5213: f64, t9483: f64, t21922: f64, t21918: f64, t21926: f64, t342: f64, t630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82409 = t8675 * t21875;
    let t82488 = t1526 * t13598 * t21911;
    let t82491 = t1526 * t9483 * t5213;
    let t82494 = t1526 * t9483 * t21922;
    let t82497 = t1526 * t9483 * t21918;
    let t82552 = t342 * t630 * t21926;
    (t82409, t82488, t82491, t82494, t82497, t82552)
}
