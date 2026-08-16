//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1077/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1077(t255: f64, t42109: f64, t2603: f64, t38953: f64, t2610: f64, t762: f64, t9802: f64, t10076: f64, t8392: f64, t754: f64, t9895: f64, t2542: f64, t737: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42350 = t42109 * t255;
    let t42358 = t38953 * t2603;
    let t42360 = t38953 * t2610;
    let t42362 = t9802 * t762;
    let t42374 = t8392 * t10076;
    let t42376 = t9895 * t754;
    let t42385 = t737 * t2542;
    (t42350, t42358, t42360, t42362, t42374, t42376, t42385)
}
