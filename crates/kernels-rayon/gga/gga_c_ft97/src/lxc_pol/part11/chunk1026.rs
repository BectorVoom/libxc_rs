//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1026/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1026(t3806: f64, t41473: f64, t701: f64, t703: f64, t9577: f64, t41448: f64, t420: f64, t1934: f64, t2405: f64) -> (f64, f64, f64) {
    let t41475 = t701 * t3806 * t41473;
    let t41477 = t703 * t9577;
    let t41480 = t701 * t420 * t41477 * t41448;
    let t41482 = t2405 * t1934;
    (t41475, t41480, t41482)
}
