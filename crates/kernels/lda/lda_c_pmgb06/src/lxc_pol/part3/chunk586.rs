//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 586/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk586<F: Float>(t117: F, t123: F, t125: F, t2825: F, t2828: F, t2831: F, t2835: F, t2840: F, t2844: F, t2846: F, t2849: F, t3467: F, t3474: F, t3478: F, t3481: F, t1282: F, t342: F) -> (F, F) {
    let t3482 = t2825 - 0.005926167098672845 * t2828 - 0.01185233419734569 * t2831 - 0.0014862827083471494 * t2835 - t2840 - t2844 - t2846 + 0.01975389032890948 * t2849 - 0.005388405304614574 * t123 * t125 * t3467 * t117 - 0.07184540406152766 * t3474 + 0.02694202652307287 * t3478 + t3481;
    let t3494 = t1282 * t342;
    (t3482, t3494)
}
