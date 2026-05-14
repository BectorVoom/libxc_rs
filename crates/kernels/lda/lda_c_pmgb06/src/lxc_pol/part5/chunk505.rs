//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 505/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk505<F: Float>(t166: F, t2653: F, t161: F, t1732: F, t2586: F, t2594: F, t2596: F, t2598: F, t2603: F, t2608: F, t2627: F, t2629: F, t2633: F, t2652: F, t2504: F, t2532: F, t2568: F) -> (F, F, F) {
    let t2654 = t166 * t2653;
    let t2656 = t161 * t2654 / 15.0;
    let t2657 = t2586 + t2594 + t2596 - t2598 + t2603 + t2608 - t2627 - t2629 - t2633 - t2652 - t2656 + t1732;
    let t2659 = t2504 + t2532 + t2568 + t2657;
    (t2654, t2656, t2659)
}
