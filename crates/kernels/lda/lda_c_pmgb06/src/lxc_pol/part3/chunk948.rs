//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 948/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk948<F: Float>(t12807: F, t1596: F, t1887: F, t3292: F, t802: F, t9626: F, t9628: F, t132: F, t435: F, t5040: F, t9633: F, t12794: F, t12798: F, t12801: F, t12803: F, t12804: F) -> (F, F, F, F, F, F, F, F) {
    let t12808 = t12807 / 15.0;
    let t12810 = t1887 * t1596 / 5.0;
    let t12812 = t802 * t3292 / 5.0;
    let t12813 = t9626 / 15.0;
    let t12814 = 2.0 / 15.0 * t9628;
    let t12816 = t132 * t435 * t5040;
    let t12817 = t12816 / 15.0;
    let t12818 = 2.0 / 15.0 * t9633;
    let t12819 = t12794 + t12798 - t12801 + t12803 + 8.0 / 81.0 * t12804 - t12808 + t12810 + t12812 - t12813 - t12814 - t12817 - t12818;
    (t12808, t12810, t12812, t12813, t12814, t12817, t12818, t12819)
}
