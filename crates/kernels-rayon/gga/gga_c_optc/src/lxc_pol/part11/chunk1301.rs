//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1301/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1301(t24321: f64, t30270: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49393: f64, t49395: f64, t56969: f64, t57027: f64, t57037: f64, t57041: f64) -> f64 {
    let t57209 = -0.18541666666666666666e-1_f64 * t57027 - 0.61805555555555555555e-1_f64 * t56969 - 0.16481481481481481482e-1_f64 * t39413 + 0.49444444444444444445e-1_f64 * t39418 + t24321 + 0.24722222222222222222e-1_f64 * t49240 - 0.74166666666666666668e-1_f64 * t49242 + 0.12361111111111111111e-1_f64 * t49393 + 0.13734567901234567901e-1_f64 * t49395 + 0.38456790123456790123e-1_f64 * t30270 + 0.12361111111111111111e0_f64 * t57037 - 0.27469135802469135803e-1_f64 * t57041;
    t57209
}
