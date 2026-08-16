//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 208/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk208(t40: f64, t559: f64, t67: f64, t62: f64, t520: f64, t522: f64, t526: f64, t531: f64) -> (f64, f64, f64, f64, f64) {
    let t560 = t40 * t559;
    let t564 = t67 * t67;
    let t565 = 1.0_f64 / t564;
    let t566 = t62 * t565;
    let t571 = -0.1176575e1_f64 * t520 - 0.516475e0_f64 * t522 - 0.2103875e0_f64 * t526 - 0.104195e0_f64 * t531;
    (t560, t564, t565, t566, t571)
}
