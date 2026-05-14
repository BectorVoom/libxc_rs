//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 763/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk763<F: Float>(t173: F, t7659: F, t184: F, t199: F, t4013: F, t4657: F, t6638: F, t6649: F, t6657: F, t7431: F, t7435: F, t7438: F, t7441: F, t7450: F, t7453: F, t203: F) -> (F, F, F, F, F) {
    let t7660 = t173 * t7659;
    let t7661 = t7660 * t184;
    let t7663 = 2.0 / 15.0 * t7661 * t199;
    let t7674 = t4013 + 0.002518888888888889 * t4657 - 0.0012594444444444445 * t6638 + 0.003778333333333333 * t6649 - 0.0018891666666666666 * t6657 + 0.002099074074074074 * t7450 - 0.007556666666666666 * t7431 + 0.003778333333333333 * t7435 + 0.011335 * t7438 - 0.011335 * t7441 + 0.0018891666666666666 * t7453;
    let t7675 = t203 * t7674;
    (t7660, t7661, t7663, t7674, t7675)
}
