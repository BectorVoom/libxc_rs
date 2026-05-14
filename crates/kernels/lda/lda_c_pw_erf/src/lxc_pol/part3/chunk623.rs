//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 623/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk623<F: Float>(t254: F, t474: F, t252: F, t1511: F, t509: F, t184: F, t199: F, t3542: F, t3493: F, t3496: F, t3499: F, t3502: F, t3505: F, t3528: F, t3530: F, t3532: F, t3534: F, t3538: F) -> (F, F, F, F, F, F, F) {
    let t3990 = t254 * t474;
    let t3992 = 8.0 / 81.0 * t252 * t3990;
    let t3993 = t1511 * t509;
    let t3994 = t3993 * t184;
    let t3996 = 4.0 / 5.0 * t3994 * t199;
    let t3997 = 0.005877407407407408 * t3542;
    let t4008 = t3997 + 0.002518888888888889 * t3530 - 0.0012594444444444445 * t3534 + 0.003778333333333333 * t3493 - 0.0018891666666666666 * t3532 + 0.002099074074074074 * t3538 - 0.007556666666666666 * t3496 + 0.003778333333333333 * t3499 + 0.011335 * t3502 - 0.011335 * t3505 + 0.0018891666666666666 * t3528;
    (t3990, t3992, t3993, t3994, t3996, t3997, t4008)
}
