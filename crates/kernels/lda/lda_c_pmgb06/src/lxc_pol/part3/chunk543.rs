//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 543/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk543<F: Float>(t2803: F, t3: F, t1338: F, t415: F, t1139: F, t118: F, t718: F, t1166: F, t81: F) -> (F, F, F, F, F) {
    let t2804 = t3 * t2803;
    let t2807 = t1338 * t415;
    let t2809 = t1139 * t118;
    let t2812 = F::new(0.1890324433388467) * t718 * t415;
    let t2813 = t81 * t1166;
    (t2804, t2807, t2809, t2812, t2813)
}
