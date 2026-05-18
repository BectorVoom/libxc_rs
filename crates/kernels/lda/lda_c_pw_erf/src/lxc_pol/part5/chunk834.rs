//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 834/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk834<F: Float>(t503: F, t7655: F, t11: F, t3997: F, t4600: F, t6545: F, t6547: F, t6549: F, t7637: F, t7641: F, t7645: F, t7649: F, t7653: F) -> (F, F, F) {
    let t7656 = t503 * t7655;
    let t7657 = t11 * t7656;
    let t7659 = t3997 + F::new(0.002518888888888889) * t4600 - F::new(0.0012594444444444445) * t6549 + F::new(0.003778333333333333) * t6545 - F::new(0.0018891666666666666) * t6547 + F::new(0.002099074074074074) * t7637 - F::new(0.007556666666666666) * t7641 + F::new(0.003778333333333333) * t7645 + F::new(0.011335) * t7649 - F::new(0.011335) * t7653 + F::new(0.0018891666666666666) * t7657;
    (t7656, t7657, t7659)
}
