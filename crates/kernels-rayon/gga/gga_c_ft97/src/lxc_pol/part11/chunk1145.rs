//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1145/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1145(t43399: f64, t43403: f64, t43407: f64, t43411: f64, t43416: f64, t43418: f64, t43422: f64, t43424: f64, t43426: f64, t43430: f64, t43433: f64, t43437: f64, t43441: f64, t43444: f64, t43448: f64) -> f64 {
    let t44096 = 2.0_f64 * t43399 + 9.0_f64 / 4.0_f64 * t43403 - t43407 - 8.0_f64 * t43411 + 4.0_f64 / 3.0_f64 * t43416 + 8.0_f64 / 9.0_f64 * t43418 + 4.0_f64 / 3.0_f64 * t43422 - 4.0_f64 / 3.0_f64 * t43424 - 4.0_f64 / 3.0_f64 * t43426 + 4.0_f64 / 3.0_f64 * t43430 + 8.0_f64 / 3.0_f64 * t43433 + 8.0_f64 * t43437 + 2.0_f64 * t43441 + 112.0_f64 / 81.0_f64 * t43444 - 2.0_f64 / 3.0_f64 * t43448;
    t44096
}
