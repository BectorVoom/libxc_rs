//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 844/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk844<F: Float>(t5076: F, t6545: F, t6547: F, t6549: F, t6551: F, t6553: F, t6555: F, t7637: F, t7649: F, t7653: F, t7657: F, t7779: F) -> F {
    let t7791 = -F::cast_from(0.21595_f64) * t7649 + F::cast_from(0.21595_f64) * t7653 - F::cast_from(0.07198333333333333_f64) * t6545 + F::cast_from(0.035991666666666665_f64) * t6547 + F::cast_from(0.023994444444444443_f64) * t6549 + F::cast_from(0.0044444444444444444_f64) * t6551 - F::cast_from(0.02666666666666667_f64) * t6553 + F::cast_from(0.013333333333333334_f64) * t6555 - F::cast_from(0.035991666666666665_f64) * t7657 - F::cast_from(0.03999074074074074_f64) * t7637 - F::cast_from(0.022222222222222223_f64) * t5076;
    let t7792 = t7779 + t7791;
    t7792
}
