//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1067/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1067<F: Float>(t161: F, t489: F, t5109: F, t4844: F, t486: F, t5105: F, t4953: F, t3005: F, t831: F, t132: F, t435: F, t4816: F) -> (F, F, F, F, F, F) {
    let t11750 = t161 * t489 * t5109;
    let t11757 = t486 * t4844;
    let t11762 = t486 * t5105;
    let t11765 = t161 * t489 * t4953;
    let t11777 = t831 * t3005;
    let t11792 = t132 * t435 * t4816;
    (t11750, t11757, t11762, t11765, t11777, t11792)
}
