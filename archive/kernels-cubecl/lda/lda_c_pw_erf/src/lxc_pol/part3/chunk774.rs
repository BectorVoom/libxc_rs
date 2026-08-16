//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 774/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk774<F: Float>(t190: F, t2061: F, t25: F, t3469: F, t3530: F, t3532: F, t3534: F, t4612: F, t4622: F, t4626: F, t4635: F, t4639: F, t4643: F, t5096: F, t5097: F, t5100: F, t5103: F, t5106: F, t5109: F, t5112: F, t5121: F) -> F {
    let t5126 = t5096 - F::cast_from(0.0022222222222222222_f64) * t25 * t5097 - F::cast_from(0.002962962962962963_f64) * t25 * t5100 - F::cast_from(0.008888888888888889_f64) * t2061 * t5103 + F::cast_from(0.013333333333333334_f64) * t25 * t5106 + F::cast_from(0.05333333333333334_f64) * t2061 * t5109 + t5112 - F::cast_from(0.023994444444444443_f64) * t4626 - F::cast_from(0.03999074074074074_f64) * t4612 - F::cast_from(0.09597777777777777_f64) * t4622 + F::cast_from(0.07198333333333333_f64) * t4639 + F::cast_from(0.2879333333333333_f64) * t4635 - F::cast_from(0.03199259259259259_f64) * t3530 + F::cast_from(0.011997222222222222_f64) * t3532 + F::cast_from(0.007998148148148148_f64) * t3534 - F::cast_from(0.013333333333333334_f64) * t190 * t3469 * t5121 - F::cast_from(0.07198333333333333_f64) * t4643;
    t5126
}
