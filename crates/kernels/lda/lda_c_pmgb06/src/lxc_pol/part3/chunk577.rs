//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 577/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk577<F: Float>(t1576: F, t2919: F, t2924: F, t2928: F, t525: F, t2933: F, t526: F, t955: F, t1583: F, t405: F, t1577: F, t163: F, t497: F, t147: F, t2913: F, t2939: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3338 = t1576 * t2919;
    let t3341 = t1576 * t2924;
    let t3344 = t525 * t2928;
    let t3347 = t525 * t2933;
    let t3350 = t955 * t526;
    let t3352 = t405 * t1583;
    let t3354 = t405 * t1577;
    let t3357 = 1.0 / t163 / t497;
    let t3358 = t147 * t3357;
    let t3359 = t3358 * t2913;
    let t3362 = t525 * t2939;
    (t3338, t3341, t3344, t3347, t3350, t3352, t3354, t3357, t3358, t3359, t3362)
}
