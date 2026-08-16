//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 864/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk864(t100: f64, t38482: f64, t104: f64, t38061: f64, t89: f64, t487: f64, t7800: f64, t179: f64, t37406: f64, t70: f64, t8119: f64, t37355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39272 = t38482 * t100;
    let t39317 = 280.0_f64 / 243.0_f64 * t89 * t38061 * t104;
    let t39345 = t487 * t7800;
    let t39417 = t179 * t37406;
    let t39430 = t70 * t8119;
    let t39431 = t179 * t37355;
    (t39272, t39317, t39345, t39417, t39430, t39431)
}
