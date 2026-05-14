//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 734/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk734<F: Float>(t2777: F, t2780: F, t5615: F, t5622: F, t5625: F, t5627: F, t5698: F, t5702: F, t5712: F, t7153: F, t7167: F, t7176: F, t6230: F, t851: F, t166: F, t161: F) -> (F, F, F, F) {
    let t7441 = -0.09451622166942335 * t5698 + 0.1890324433388467 * t5702 - 0.07184540406152766 * t5712 - 0.1890324433388467 * t5627 + 0.01975389032890948 * t5615 - 0.01185233419734569 * t5622 - 0.0014862827083471494 * t5625 + 0.02694202652307287 * t7167 - 0.09451622166942335 * t7176 + 0.09451622166942335 * t7153 + t2777 - t2780;
    let t7442 = t6230 * t851;
    let t7443 = t166 * t7442;
    let t7445 = t161 * t7443 / 10.0;
    (t7441, t7442, t7443, t7445)
}
