//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1078/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1078<F: Float>(t132: F, t435: F, t4978: F, t1596: F, t1887: F, t3292: F, t802: F, t9626: F, t9628: F, t5040: F, t9633: F, t12794: F, t12798: F, t12801: F, t12803: F, t12804: F) -> (F, F, F, F, F, F, F, F) {
    let t12807 = t132 * t435 * t4978;
    let t12808 = t12807 / F::cast_from(15.0_f64);
    let t12810 = t1887 * t1596 / F::cast_from(5.0_f64);
    let t12812 = t802 * t3292 / F::cast_from(5.0_f64);
    let t12813 = t9626 / F::cast_from(15.0_f64);
    let t12814 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t9628;
    let t12816 = t132 * t435 * t5040;
    let t12817 = t12816 / F::cast_from(15.0_f64);
    let t12818 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t9633;
    let t12819 = t12794 + t12798 - t12801 + t12803 + F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t12804 - t12808 + t12810 + t12812 - t12813 - t12814 - t12817 - t12818;
    (t12808, t12810, t12812, t12813, t12814, t12817, t12818, t12819)
}
