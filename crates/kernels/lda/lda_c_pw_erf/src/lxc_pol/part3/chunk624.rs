//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 624/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk624<F: Float>(t173: F, t4008: F, t184: F, t199: F, t3638: F, t3625: F, t3627: F, t3629: F, t3631: F, t3635: F, t3641: F, t3644: F, t3646: F, t3649: F, t3652: F, t203: F) -> (F, F, F, F, F, F) {
    let t4009 = t173 * t4008;
    let t4010 = t4009 * t184;
    let t4012 = 2.0 / 15.0 * t4010 * t199;
    let t4013 = 0.005877407407407408 * t3638;
    let t4024 = t4013 + 0.002518888888888889 * t3627 - 0.0012594444444444445 * t3631 + 0.003778333333333333 * t3646 - 0.0018891666666666666 * t3629 + 0.002099074074074074 * t3635 - 0.007556666666666666 * t3649 + 0.003778333333333333 * t3652 + 0.011335 * t3641 - 0.011335 * t3644 + 0.0018891666666666666 * t3625;
    let t4025 = t203 * t4024;
    (t4009, t4010, t4012, t4013, t4024, t4025)
}
