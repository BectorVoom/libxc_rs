//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1039/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1039(t46256: f64, t46320: f64, t57718: f64, t59170: f64, t73439: f64, t73442: f64, t74307: f64, t74374: f64, t74377: f64, t86289: f64, t86297: f64, t86300: f64, t86303: f64, t86306: f64, t86309: f64) -> f64 {
    let t86402 = 112.0_f64 / 81.0_f64 * t46256 - t86289 / 3.0_f64 - 8.0_f64 / 27.0_f64 * t57718 + 112.0_f64 / 243.0_f64 * t46320 + 8.0_f64 / 3.0_f64 * t73439 + 4.0_f64 / 9.0_f64 * t73442 - 8.0_f64 / 9.0_f64 * t74307 + 8.0_f64 / 27.0_f64 * t74374 - 16.0_f64 / 9.0_f64 * t86297 - 8.0_f64 / 9.0_f64 * t86300 - 4.0_f64 / 3.0_f64 * t86303 + 4.0_f64 / 9.0_f64 * t86306 - 4.0_f64 / 3.0_f64 * t86309 + 40.0_f64 / 243.0_f64 * t74377 - 8.0_f64 / 9.0_f64 * t59170;
    t86402
}
