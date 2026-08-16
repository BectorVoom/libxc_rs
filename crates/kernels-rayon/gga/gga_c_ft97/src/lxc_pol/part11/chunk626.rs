//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 626/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk626(t634: f64, t8640: f64, t2253: f64, t2277: f64, t2261: f64, t2284: f64, t2259: f64, t72: f64, t7765: f64, t3621: f64, t7789: f64, t422: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8641 = t8640 * t634;
    let t8643 = t2253 * t2277;
    let t8645 = t2253 * t2261;
    let t8647 = t2253 * t2284;
    let t8650 = t72 * t2259 * t7765;
    let t8652 = t3621 * t7789;
    let t8654 = t422 * t639;
    (t8641, t8643, t8645, t8647, t8650, t8652, t8654)
}
