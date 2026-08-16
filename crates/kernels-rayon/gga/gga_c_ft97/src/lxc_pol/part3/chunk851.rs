//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 851/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk851(t2086: f64, t4778: f64, t590: f64, t91: f64, t4753: f64, t9252: f64, t3491: f64, t3526: f64, t16710: f64, t16714: f64, t16717: f64, t16721: f64, t16724: f64, t16727: f64, t16730: f64, t16734: f64) -> (f64, f64, f64, f64) {
    let t17235 = t2086 * t4778;
    let t17237 = t91 * t17235 * t590;
    let t17239 = t9252 * t4753;
    let t17241 = t91 * t17239 * t590;
    let t17244 = t91 * t3491 * t3526;
    let t17246 = 4.0_f64 / 3.0_f64 * t16710 - 2.0_f64 / 3.0_f64 * t16714 - 2.0_f64 * t16717 + 2.0_f64 / 9.0_f64 * t16721 + 4.0_f64 / 3.0_f64 * t16724 - 10.0_f64 / 27.0_f64 * t16727 - 8.0_f64 / 9.0_f64 * t16730 + 2.0_f64 / 3.0_f64 * t16734 - t17237 / 4.0_f64 + 3.0_f64 / 8.0_f64 * t17241 - t17244 / 2.0_f64;
    (t17237, t17241, t17244, t17246)
}
