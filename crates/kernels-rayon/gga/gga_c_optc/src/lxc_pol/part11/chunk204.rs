//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 204/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk204(t544: f64, t88: f64, t37: f64, t34: f64, t67: f64, t62: f64, t520: f64, t522: f64, t526: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t546 = 4.0_f64 * t544 * t88;
    let t547 = 1.0_f64 / t37;
    let t548 = t34 * t547;
    let t564 = t67 * t67;
    let t565 = 1.0_f64 / t564;
    let t566 = t62 * t565;
    let t571 = -0.1176575e1_f64 * t520 - 0.516475e0_f64 * t522 - 0.2103875e0_f64 * t526 - 0.104195e0_f64 * t531;
    (t546, t547, t548, t564, t565, t566, t571)
}
