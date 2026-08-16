//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 540/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk540(t1189: f64, t3477: f64, t3440: f64, t3445: f64, t3449: f64, t3453: f64, t3455: f64, t3457: f64, t3461: f64, t3467: f64, t3469: f64, t3471: f64, t3475: f64) -> (f64, f64) {
    let t3478 = t3477 * t1189;
    let t3480 = t3440 / 96.0_f64 - t3445 / 128.0_f64 - t3449 / 192.0_f64 + t3453 / 256.0_f64 + t3455 / 24.0_f64 - t3457 / 96.0_f64 - t3461 / 16.0_f64 + t3467 / 8.0_f64 - t3469 / 3.0_f64 + t3471 / 12.0_f64 - t3475 / 8.0_f64 + t3478 / 128.0_f64;
    (t3478, t3480)
}
