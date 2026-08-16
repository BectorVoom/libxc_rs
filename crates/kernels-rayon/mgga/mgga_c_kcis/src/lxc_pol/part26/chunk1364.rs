//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1364/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1364(t102197: f64, t102205: f64, t28369: f64, t28485: f64, t28489: f64, t28495: f64, t98119: f64, t98365: f64, t98381: f64, t98383: f64, t98387: f64, t98388: f64, t98390: f64) -> f64 {
    let t103418 = -0.61836467013888888889e-4_f64 * t98365 + 0.55273148148148148147e-3_f64 * t102197 + t98381 + 0.73697530864197530862e-3_f64 * t102205 - t98383 - t98387 + 0.12356481481481481482e-2_f64 * t98388 + 0.30891203703703703704e-3_f64 * t98390 + 0.46336805555555555556e-3_f64 * t28369 * t28485 + 0.92673611111111111112e-3_f64 * t28369 * t28489 + 0.61836467013888888889e-4_f64 * t98119 * t28485 - 0.61782407407407407408e-3_f64 * t28369 * t28495;
    t103418
}
