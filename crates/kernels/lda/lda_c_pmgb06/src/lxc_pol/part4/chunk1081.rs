//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1081/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1081<F: Float>(t136: F, t1438: F, t3098: F, t441: F, t1881: F, t642: F, t1548: F, t1887: F, t2857: F, t802: F, t161: F, t3004: F, t852: F) -> (F, F, F, F, F, F) {
    let t12402 = t136 * t1438;
    let t12406 = t441 * t3098;
    let t12429 = F::cast_from(48.0_f64) * t1881 * t642;
    let t12447 = t1887 * t1548;
    let t12449 = t802 * t2857;
    let t12456 = t161 * t3004 * t852;
    (t12402, t12406, t12429, t12447, t12449, t12456)
}
