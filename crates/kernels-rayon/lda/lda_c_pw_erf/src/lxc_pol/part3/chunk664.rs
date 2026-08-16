//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 664/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk664(t254: f64, t474: f64, t252: f64, t1511: f64, t509: f64, t184: f64, t199: f64, t3542: f64, t3493: f64, t3496: f64, t3499: f64, t3502: f64, t3505: f64, t3528: f64, t3530: f64, t3532: f64, t3534: f64, t3538: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3990 = t254 * t474;
    let t3992 = 8.0_f64 / 81.0_f64 * t252 * t3990;
    let t3993 = t1511 * t509;
    let t3994 = t3993 * t184;
    let t3996 = 4.0_f64 / 5.0_f64 * t3994 * t199;
    let t3997 = 0.005877407407407408_f64 * t3542;
    let t4008 = t3997 + 0.002518888888888889_f64 * t3530 - 0.0012594444444444445_f64 * t3534 + 0.003778333333333333_f64 * t3493 - 0.0018891666666666666_f64 * t3532 + 0.002099074074074074_f64 * t3538 - 0.007556666666666666_f64 * t3496 + 0.003778333333333333_f64 * t3499 + 0.011335_f64 * t3502 - 0.011335_f64 * t3505 + 0.0018891666666666666_f64 * t3528;
    (t3990, t3992, t3993, t3994, t3996, t3997, t4008)
}
