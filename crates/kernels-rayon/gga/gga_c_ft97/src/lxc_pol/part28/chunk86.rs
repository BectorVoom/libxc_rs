//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 86/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk86(t72: f64, t81: f64, t342: f64, t343: f64, t88: f64, t13: f64, t14: f64, t12: f64, t10: f64, t83: f64, t174: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t344 = t72 * t81;
    let t348 = t88 - t342 * t343 * t344 / 4.0_f64;
    let t349 = t14 * t13;
    let t350 = 1.0_f64 / t349;
    let t351 = t12 * t350;
    let t353 = t10 * t351 * t83;
    let t354 = t353 / 18.0_f64;
    let t355 = 1.0_f64 / t174;
    (t344, t348, t350, t351, t353, t354, t355)
}
