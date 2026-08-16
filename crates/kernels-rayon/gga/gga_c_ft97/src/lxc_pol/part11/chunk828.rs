//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 828/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk828(t5517: f64, t66: f64, t37: f64, t401: f64, t78: f64, t1299: f64, t1664: f64, t139: f64, t39: f64, t527: f64, t135: f64, t1995: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22696 = t5517 * t66;
    let t22833 = t37 * t401;
    let t22834 = t22833 * t78;
    let t22852 = t1664 * t1299;
    let t23809 = t139 * t39;
    let t23810 = t527 * t23809;
    let t23831 = t1995 * t135;
    (t22696, t22834, t22852, t23809, t23810, t23831)
}
