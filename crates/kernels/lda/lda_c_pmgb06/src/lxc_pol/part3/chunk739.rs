//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 739/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk739<F: Float>(t4637: F, t819: F, t955: F, t146: F, t3082: F, t3084: F, t3086: F, t3088: F, t3365: F, t3428: F, t4639: F, t4647: F, t4652: F, t4657: F, t4661: F, t4665: F, t4670: F, t4674: F, t4678: F, t4989: F) -> F {
    let t5002 = F::new(0.015996296296296297) * t4637;
    let t5003 = t955 * t819;
    let t5005 = -F::new(0.008888888888888889) * t3428 - F::new(0.023994444444444443) * t3086 - F::new(0.03199259259259259) * t3082 + F::new(0.011997222222222222) * t3088 + F::new(0.007998148148148148) * t3084 - F::new(0.013333333333333334) * t146 * t3365 * t4989 - F::new(0.07198333333333333) * t4678 - F::new(0.21595) * t4665 + F::new(0.14396666666666666) * t4652 - F::new(0.023994444444444443) * t4661 - F::new(0.03999074074074074) * t4647 - F::new(0.09597777777777777) * t4657 + F::new(0.07198333333333333) * t4674 + F::new(0.2879333333333333) * t4670 - F::new(0.047988888888888886) * t4639 + t5002 - F::new(0.007407407407407408) * t5003;
    t5005
}
