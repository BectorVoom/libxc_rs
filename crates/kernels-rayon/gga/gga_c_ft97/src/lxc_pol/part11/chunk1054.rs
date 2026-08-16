//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1054/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1054(t27: f64, t41751: f64, t241: f64, t41536: f64, t41448: f64, t89: f64, t2336: f64, t9703: f64, t2345: f64, t9717: f64, t681: f64, t9713: f64) -> (f64, f64, f64, f64, f64) {
    let t41911 = t27 * t41751;
    let t41912 = t241 * t41536;
    let t41915 = t89 * t41911 * t41912 * t41448;
    let t41918 = t89 * t2336 * t9703;
    let t41922 = t89 * t2345 * t9717 * t41448;
    let t41925 = t89 * t681 * t9713;
    (t41911, t41915, t41918, t41922, t41925)
}
