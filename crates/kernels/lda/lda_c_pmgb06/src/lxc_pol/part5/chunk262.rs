//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 262/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk262<F: Float>(t161: F, t844: F, t525: F, t838: F, t103: F, t519: F, t523: F, t840: F) -> (F, F, F) {
    let t846 = t161 * t844 / F::new(30.0);
    let t848 = t525 * t838;
    let t851 = -t519 - F::cast_from(0.035991666666666665_f64) * t840 - t523 - F::cast_from(0.006666666666666667_f64) * t103 * t848;
    (t846, t848, t851)
}
