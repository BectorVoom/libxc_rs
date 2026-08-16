//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 882/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk882(t2492: f64, t2568: f64, t255: f64, t42109: f64, t762: f64, t9802: f64, t42163: f64, t761: f64, t9570: f64, t241: f64, t41752: f64, t192: f64, t33300: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42339 = t2492 * t2568;
    let t42350 = t42109 * t255;
    let t42362 = t9802 * t762;
    let t42409 = t42163 * t255;
    let t42416 = t761 * t9570;
    let t42469 = t41752 * t241;
    let t42500 = t192 * t33300;
    (t42339, t42350, t42362, t42409, t42416, t42469, t42500)
}
