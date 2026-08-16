//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 830/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk830(t294: f64, t7639: f64, t13: f64, t21: f64, t2: f64, t7242: f64, t113: f64, t7806: f64, t8494: f64, t446: f64, t7793: f64, t376: f64, t7756: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33828 = 1.0_f64 / t7639 / t294;
    let t36377 = t13 * t21;
    let t36452 = t7242 * t2;
    let t36827 = t13 * t113;
    let t37252 = t7806 * t8494;
    let t37254 = t446 * t7793 * t37252;
    let t37257 = t89 * t376 * t7756;
    (t33828, t36377, t36452, t36827, t37252, t37254, t37257)
}
