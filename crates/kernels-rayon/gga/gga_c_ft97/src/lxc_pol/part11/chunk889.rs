//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 889/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk889(t38260: f64, t378: f64, t7241: f64, t358: f64, t363: f64, t7751: f64, t446: f64, t1586: f64, t1642: f64, t1588: f64, t1643: f64, t432: f64, t7959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38261 = 4.0_f64 / 9.0_f64 * t38260;
    let t38262 = t378 * t7241;
    let t38264 = t7751 * t358 * t363;
    let t38266 = t446 * t38262 * t38264;
    let t38268 = t1642 * t1586;
    let t38269 = t1643 * t1588;
    let t38271 = t446 * t38268 * t38269;
    let t38273 = t7959 * t432;
    (t38261, t38264, t38266, t38269, t38271, t38273)
}
