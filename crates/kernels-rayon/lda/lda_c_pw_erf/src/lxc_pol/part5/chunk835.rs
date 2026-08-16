//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 835/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk835(t173: f64, t7659: f64, t184: f64, t199: f64, t4013: f64, t4657: f64, t6638: f64, t6649: f64, t6657: f64, t7431: f64, t7435: f64, t7438: f64, t7441: f64, t7450: f64, t7453: f64) -> (f64, f64, f64, f64) {
    let t7660 = t173 * t7659;
    let t7661 = t7660 * t184;
    let t7663 = 2.0_f64 / 15.0_f64 * t7661 * t199;
    let t7674 = t4013 + 0.002518888888888889_f64 * t4657 - 0.0012594444444444445_f64 * t6638 + 0.003778333333333333_f64 * t6649 - 0.0018891666666666666_f64 * t6657 + 0.002099074074074074_f64 * t7450 - 0.007556666666666666_f64 * t7431 + 0.003778333333333333_f64 * t7435 + 0.011335_f64 * t7438 - 0.011335_f64 * t7441 + 0.0018891666666666666_f64 * t7453;
    (t7660, t7661, t7663, t7674)
}
