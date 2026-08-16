//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 810/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk810(t11175: f64, t9: f64, t534: f64, t7858: f64, t371: f64, t7876: f64, t25: f64, t78: f64, t1602: f64, t122: f64, t173: f64, t1736: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11176 = t9 * t11175;
    let t11209 = t534 * t7858;
    let t11232 = t371 * t7876;
    let t11240 = t78 * t25;
    let t11241 = t1602 * t11240;
    let t11245 = t78 * t122;
    let t11246 = t1602 * t11245;
    let t11262 = t173 * t1736;
    (t11176, t11209, t11232, t11240, t11241, t11246, t11262)
}
