//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 774/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk774(t190: f64, t2061: f64, t25: f64, t3469: f64, t3530: f64, t3532: f64, t3534: f64, t4612: f64, t4622: f64, t4626: f64, t4635: f64, t4639: f64, t4643: f64, t5096: f64, t5097: f64, t5100: f64, t5103: f64, t5106: f64, t5109: f64, t5112: f64, t5121: f64) -> f64 {
    let t5126 = t5096 - 0.0022222222222222222_f64 * t25 * t5097 - 0.002962962962962963_f64 * t25 * t5100 - 0.008888888888888889_f64 * t2061 * t5103 + 0.013333333333333334_f64 * t25 * t5106 + 0.05333333333333334_f64 * t2061 * t5109 + t5112 - 0.023994444444444443_f64 * t4626 - 0.03999074074074074_f64 * t4612 - 0.09597777777777777_f64 * t4622 + 0.07198333333333333_f64 * t4639 + 0.2879333333333333_f64 * t4635 - 0.03199259259259259_f64 * t3530 + 0.011997222222222222_f64 * t3532 + 0.007998148148148148_f64 * t3534 - 0.013333333333333334_f64 * t190 * t3469 * t5121 - 0.07198333333333333_f64 * t4643;
    t5126
}
