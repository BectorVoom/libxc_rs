//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1366/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1366(t13793: f64, t56320: f64, t14657: f64, t53245: f64, t52996: f64, t14469: f64, t53229: f64, t13888: f64, t2408: f64, t54464: f64, t57449: f64, t57454: f64, t57458: f64, t57462: f64, t57468: f64, t57472: f64, t57474: f64, t57476: f64, t57480: f64, t9283: f64, t9926: f64) -> f64 {
    let t57482 = t56320 * t13793;
    let t57484 = t14657 * t53245;
    let t57486 = t14657 * t52996;
    let t57488 = t53229 * t14469;
    let t57490 = t57449 / 96.0_f64 - t57454 / 1536.0_f64 + t57458 / 96.0_f64 + t57462 / 3072.0_f64 - t2408 * t9283 * t13888 * t9926 / 12.0_f64 + t57468 / 96.0_f64 - 7.0_f64 / 288.0_f64 * t57472 - t54464 - t57474 / 48.0_f64 - 7.0_f64 / 2304.0_f64 * t57476 - t57480 / 96.0_f64 - t57482 / 48.0_f64 - t57484 / 24.0_f64 - t57486 / 24.0_f64 + 7.0_f64 / 72.0_f64 * t57488;
    t57490
}
