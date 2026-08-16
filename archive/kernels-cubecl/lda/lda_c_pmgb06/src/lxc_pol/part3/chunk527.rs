//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 527/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk527<F: Float>(t2209: F, t77: F, t315: F, t794: F, t123: F, t199: F, t125: F, t1798: F, t722: F, t868: F, t395: F, t902: F) -> (F, F, F, F, F, F) {
    let t2276 = t77 * t2209;
    let t2281 = t315 * t794;
    let t2283 = t123 * t2281 * t199;
    let t2285 = t125 * t1798;
    let t2293 = t123 * t722 * t868;
    let t2302 = t395 * t902;
    (t2276, t2281, t2283, t2285, t2293, t2302)
}
