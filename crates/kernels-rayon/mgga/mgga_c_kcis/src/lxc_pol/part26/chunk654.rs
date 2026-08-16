//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 654/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk654(t7311: f64, t7315: f64, t7319: f64, t7323: f64, t7330: f64, t7333: f64, t7336: f64, t7339: f64, t7383: f64, t7387: f64, t7390: f64, t7394: f64) -> f64 {
    let t7396 = -t7311 / 72.0_f64 + t7315 / 24.0_f64 - t7319 / 128.0_f64 - t7323 / 256.0_f64 - 19.0_f64 / 144.0_f64 * t7330 + t7333 / 18.0_f64 + t7336 / 3.0_f64 - t7339 / 12.0_f64 + t7383 / 16.0_f64 + 11.0_f64 / 18.0_f64 * t7387 - 2.0_f64 / 9.0_f64 * t7390 + t7394 / 8.0_f64;
    t7396
}
