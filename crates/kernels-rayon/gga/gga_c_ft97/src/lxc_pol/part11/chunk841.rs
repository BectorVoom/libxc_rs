//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 841/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk841(t37347: f64, t37303: f64, t37308: f64, t37313: f64, t37317: f64, t37322: f64, t37326: f64, t37328: f64, t37330: f64, t37332: f64, t37335: f64, t37336: f64, t37340: f64, t37344: f64) -> f64 {
    let t37348 = 8.0_f64 / 81.0_f64 * t37347;
    let t37349 = 4.0_f64 / 9.0_f64 * t37303 + 20.0_f64 / 81.0_f64 * t37308 - 10.0_f64 / 27.0_f64 * t37313 - 2.0_f64 * t37317 + 4.0_f64 / 3.0_f64 * t37322 + 2.0_f64 / 9.0_f64 * t37326 - 4.0_f64 / 9.0_f64 * t37328 + 4.0_f64 / 27.0_f64 * t37330 - 4.0_f64 / 27.0_f64 * t37332 + t37335 - 2.0_f64 / 9.0_f64 * t37336 + 4.0_f64 / 3.0_f64 * t37340 - t37344 - t37348;
    t37349
}
