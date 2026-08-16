//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 516/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk516(t737: f64, t762: f64, t2486: f64, t2492: f64, t265: f64, t9802: f64, t1471: f64, t4092: f64, t2725: f64, t6: f64, t285: f64, t1200: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14182 = t737 * t762;
    let t14187 = t2486 * t762;
    let t14196 = t2492 * t265;
    let t14200 = t9802 * t265;
    let t14721 = t4092 * t1471;
    let t14728 = t2725 * t6;
    let t14729 = t285 * t14728;
    let t14742 = t1200 * t14728;
    (t14182, t14187, t14196, t14200, t14721, t14729, t14742)
}
