//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1047/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1047(t26467: f64, t26470: f64, t26431: f64, t26434: f64, t26437: f64, t26441: f64, t26444: f64, t26448: f64, t26451: f64, t26454: f64, t26457: f64, t26460: f64, t26468: f64) -> f64 {
    let t26471 = t26470 * t26467;
    let t26473 = -0.5405960648148148148e-2_f64 * t26431 + 0.18571777777777777777e-1_f64 * t26434 + 0.69644166666666666665e-2_f64 * t26437 + 0.13928833333333333333e-1_f64 * t26441 - 0.13928833333333333333e-1_f64 * t26444 - 0.69644166666666666665e-2_f64 * t26448 + 0.32435763888888888888e-2_f64 * t26451 - 0.18571777777777777777e-1_f64 * t26454 + 0.21667074074074074073e-1_f64 * t26457 - 0.69505208333333333333e-3_f64 * t26460 - 0.13901041666666666667e-2_f64 * t26468 - 0.18550940104166666667e-3_f64 * t26471;
    t26473
}
