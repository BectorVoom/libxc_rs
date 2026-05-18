//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1265/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1265<F: Float>(t348: F, t7354: F, t9763: F, t34: F, t6417: F, t11: F, t1243: F, t21196: F, t1953: F, t21137: F, t22277: F, t503: F) -> (F, F, F, F, F) {
    let t22713 = t9763 * t7354 * t348;
    let t22717 = t6417 * t34;
    let t22722 = t11 * t1243 * t21196;
    let t22725 = t1953 * t1243 * t21137;
    let t22728 = t11 * t503 * t22277;
    (t22713, t22717, t22722, t22725, t22728)
}
