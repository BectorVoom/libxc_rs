//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 739/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk739(t4637: f64, t819: f64, t955: f64, t146: f64, t3082: f64, t3084: f64, t3086: f64, t3088: f64, t3365: f64, t3428: f64, t4639: f64, t4647: f64, t4652: f64, t4657: f64, t4661: f64, t4665: f64, t4670: f64, t4674: f64, t4678: f64, t4989: f64) -> f64 {
    let t5002 = 0.015996296296296297_f64 * t4637;
    let t5003 = t955 * t819;
    let t5005 = -0.008888888888888889_f64 * t3428 - 0.023994444444444443_f64 * t3086 - 0.03199259259259259_f64 * t3082 + 0.011997222222222222_f64 * t3088 + 0.007998148148148148_f64 * t3084 - 0.013333333333333334_f64 * t146 * t3365 * t4989 - 0.07198333333333333_f64 * t4678 - 0.21595_f64 * t4665 + 0.14396666666666666_f64 * t4652 - 0.023994444444444443_f64 * t4661 - 0.03999074074074074_f64 * t4647 - 0.09597777777777777_f64 * t4657 + 0.07198333333333333_f64 * t4674 + 0.2879333333333333_f64 * t4670 - 0.047988888888888886_f64 * t4639 + t5002 - 0.007407407407407408_f64 * t5003;
    t5005
}
