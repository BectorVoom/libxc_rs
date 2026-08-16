//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 699/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk699(t447: f64, t4623: f64, t925: f64, t4589: f64, t942: f64, t452: f64, t488: f64, t110: f64, t20035: f64, t16052: f64, t1902: f64, t20208: f64, t3187: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20276 = t447 * t4623 * t925;
    let t20279 = t942 * t4589;
    let t20281 = t452 * t488 * t20279;
    let t20284 = t447 * t110 * t20035;
    let t20287 = t16052 * t925;
    let t20288 = t1902 * t20287;
    let t20291 = t3187 * t20208;
    (t20276, t20279, t20281, t20284, t20287, t20288, t20291)
}
