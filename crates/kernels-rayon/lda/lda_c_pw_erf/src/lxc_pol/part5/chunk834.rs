//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 834/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk834(t503: f64, t7655: f64, t11: f64, t3997: f64, t4600: f64, t6545: f64, t6547: f64, t6549: f64, t7637: f64, t7641: f64, t7645: f64, t7649: f64, t7653: f64) -> (f64, f64, f64) {
    let t7656 = t503 * t7655;
    let t7657 = t11 * t7656;
    let t7659 = t3997 + 0.002518888888888889_f64 * t4600 - 0.0012594444444444445_f64 * t6549 + 0.003778333333333333_f64 * t6545 - 0.0018891666666666666_f64 * t6547 + 0.002099074074074074_f64 * t7637 - 0.007556666666666666_f64 * t7641 + 0.003778333333333333_f64 * t7645 + 0.011335_f64 * t7649 - 0.011335_f64 * t7653 + 0.0018891666666666666_f64 * t7657;
    (t7656, t7657, t7659)
}
