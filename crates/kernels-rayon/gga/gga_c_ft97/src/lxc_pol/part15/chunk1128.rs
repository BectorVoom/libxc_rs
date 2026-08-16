//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1128/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1128(t41536: f64, t88252: f64, t41752: f64, t92: f64, t668: f64, t86571: f64, t683: f64, t41745: f64, t52453: f64, t66197: f64, t66221: f64, t80029: f64, t80031: f64, t80087: f64, t80089: f64, t80091: f64) -> (f64, f64, f64, f64, f64) {
    let t88726 = t41536 * t88252;
    let t88728 = t92 * t41752 * t88726;
    let t88730 = t668 * t86571;
    let t88732 = t92 * t683 * t88730;
    let t88734 = -16.0_f64 / 9.0_f64 * t80087 + 8.0_f64 / 3.0_f64 * t80089 + 112.0_f64 / 81.0_f64 * t52453 + 8.0_f64 / 9.0_f64 * t80029 - 8.0_f64 / 3.0_f64 * t80031 + 40.0_f64 / 81.0_f64 * t80091 + 16.0_f64 / 9.0_f64 * t66221 - 16.0_f64 / 27.0_f64 * t66197 + t41745 - 80.0_f64 / 81.0_f64 * t88728 - t88732 / 3.0_f64;
    (t88726, t88728, t88730, t88732, t88734)
}
