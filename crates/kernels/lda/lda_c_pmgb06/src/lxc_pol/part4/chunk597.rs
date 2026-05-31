//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 597/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk597<F: Float>(t2648: F, t465: F, t137: F, t132: F, t2093: F, t851: F, t166: F, t161: F, t1732: F, t2586: F, t2594: F, t2596: F, t2598: F, t2603: F, t2608: F, t2627: F, t2629: F, t2633: F) -> (F, F, F, F, F, F, F) {
    let t2649 = t465 * t2648;
    let t2650 = t137 * t2649;
    let t2652 = t132 * t2650 / F::cast_from(30.0_f64);
    let t2653 = t2093 * t851;
    let t2654 = t166 * t2653;
    let t2656 = t161 * t2654 / F::cast_from(15.0_f64);
    let t2657 = t2586 + t2594 + t2596 - t2598 + t2603 + t2608 - t2627 - t2629 - t2633 - t2652 - t2656 + t1732;
    (t2649, t2650, t2652, t2653, t2654, t2656, t2657)
}
