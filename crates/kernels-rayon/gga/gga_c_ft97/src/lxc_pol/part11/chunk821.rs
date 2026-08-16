//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 821/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk821(t2492: f64, t265: f64, t9802: f64, t332: f64, t505: f64, t2440: f64, t327: f64, t10845: f64, t2347: f64, t2360: f64, t2923: f64, t287: f64, t4061: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14196 = t2492 * t265;
    let t14200 = t9802 * t265;
    let t14408 = t332 * t505;
    let t14487 = t2440 * t327;
    let t14514 = t10845 * t2347;
    let t14519 = t2923 * t2360;
    let t14763 = t4061 * t287;
    (t14196, t14200, t14408, t14487, t14514, t14519, t14763)
}
