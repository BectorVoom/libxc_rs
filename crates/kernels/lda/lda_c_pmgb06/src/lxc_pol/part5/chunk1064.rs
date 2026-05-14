//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1064/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1064<F: Float>(t11964: F, t12038: F, t19694: F, t19696: F, t19697: F, t19698: F, t9410: F, t9412: F, t9417: F, t9422: F, t9426: F, t9429: F, t12113: F, t19699: F, t19700: F, t19701: F, t19705: F, t19706: F, t19707: F, t19708: F, t19709: F, t19712: F, t19714: F, t19716: F) -> (F, F) {
    let t21925 = t9410 + t9412 - t9417 + 2.0 / 3.0 * t9422 + t11964 + 8.0 / 81.0 * t9426 + t9429 - t12038 - t19694 - t19696 + t19697 + t19698;
    let t21927 = -t19699 - t19700 - t19701 + t19705 - t19706 - t19707 + t12113 + t19708 - t19709 - t19712 - t19714 - t19716;
    (t21925, t21927)
}
