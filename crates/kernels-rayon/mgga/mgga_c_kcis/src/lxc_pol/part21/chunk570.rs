//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 570/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk570(t3440: f64, t3445: f64, t3449: f64, t3453: f64, t3455: f64, t3457: f64, t3461: f64, t3467: f64, t3469: f64, t3471: f64, t3475: f64, t3478: f64) -> f64 {
    let t3698 = 0.26979166666666666666e-1_f64 * t3440 - 0.20234375e-1_f64 * t3445 - 0.13489583333333333333e-1_f64 * t3449 + 0.101171875e-1_f64 * t3453 + 0.10791666666666666667e0_f64 * t3455 - 0.26979166666666666666e-1_f64 * t3457 - 0.9375e-1_f64 * t3461 + 0.1875e0_f64 * t3467 - 0.5e0_f64 * t3469 + 0.125e0_f64 * t3471 - 0.1875e0_f64 * t3475 + 0.20234375e-1_f64 * t3478;
    t3698
}
