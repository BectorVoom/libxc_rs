//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1161/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1161(t29459: f64, t29461: f64, t29463: f64, t29466: f64, t29468: f64, t29471: f64, t29473: f64, t29475: f64, t29477: f64, t29480: f64, t29482: f64, t29484: f64) -> f64 {
    let t29486 = t29459 / 128.0_f64 + 11.0_f64 / 18.0_f64 * t29461 - 2.0_f64 / 9.0_f64 * t29463 - t29466 / 16.0_f64 - t29468 / 8.0_f64 - t29471 / 72.0_f64 - t29473 / 288.0_f64 + t29475 / 16.0_f64 - t29477 / 96.0_f64 + t29480 / 24.0_f64 - t29482 / 3.0_f64 + t29484 / 12.0_f64;
    t29486
}
