//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 611/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk611<F: Float>(t1623: F, t405: F, t3082: F, t3084: F, t3086: F, t3088: F, t3095: F, t3101: F, t3106: F, t3110: F, t3113: F, t3118: F) -> (F, F) {
    let t3428 = t405 * t1623;
    let t3440 = -F::cast_from(0.02666666666666667_f64) * t3428 - F::cast_from(0.07198333333333333_f64) * t3086 + F::cast_from(0.14396666666666666_f64) * t3101 - F::cast_from(0.07198333333333333_f64) * t3106 - F::cast_from(0.21595_f64) * t3110 + F::cast_from(0.21595_f64) * t3113 - F::cast_from(0.047988888888888886_f64) * t3082 + F::cast_from(0.035991666666666665_f64) * t3088 + F::cast_from(0.023994444444444443_f64) * t3084 - F::cast_from(0.03999074074074074_f64) * t3095 - F::cast_from(0.035991666666666665_f64) * t3118;
    (t3428, t3440)
}
