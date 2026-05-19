//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 755/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk755<F: Float>(t4637: F, t819: F, t955: F, t146: F, t3082: F, t3084: F, t3086: F, t3088: F, t3365: F, t3428: F, t4639: F, t4647: F, t4652: F, t4657: F, t4661: F, t4665: F, t4670: F, t4674: F, t4678: F, t4989: F) -> (F, F, F) {
    let t5002 = F::cast_from(0.015996296296296297_f64) * t4637;
    let t5003 = t955 * t819;
    let t5005 = -F::cast_from(0.008888888888888889_f64) * t3428 - F::cast_from(0.023994444444444443_f64) * t3086 - F::cast_from(0.03199259259259259_f64) * t3082 + F::cast_from(0.011997222222222222_f64) * t3088 + F::cast_from(0.007998148148148148_f64) * t3084 - F::cast_from(0.013333333333333334_f64) * t146 * t3365 * t4989 - F::cast_from(0.07198333333333333_f64) * t4678 - F::new(0.21595) * t4665 + F::cast_from(0.14396666666666666_f64) * t4652 - F::cast_from(0.023994444444444443_f64) * t4661 - F::cast_from(0.03999074074074074_f64) * t4647 - F::cast_from(0.09597777777777777_f64) * t4657 + F::cast_from(0.07198333333333333_f64) * t4674 + F::cast_from(0.2879333333333333_f64) * t4670 - F::cast_from(0.047988888888888886_f64) * t4639 + t5002 - F::cast_from(0.007407407407407408_f64) * t5003;
    (t5002, t5003, t5005)
}
