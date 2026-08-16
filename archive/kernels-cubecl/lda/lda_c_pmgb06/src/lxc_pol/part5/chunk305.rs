//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 305/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk305<F: Float>(t1092: F, t248: F, t643: F, t654: F, t687: F, t246: F, t80: F) -> (F, F, F, F, F) {
    let t1093 = t248 * t1092;
    let t1095 = t643 * t654;
    let t1098 = F::cast_from(8.0_f64) * t643 * t687;
    let t1099 = t246 * t80;
    let t1100 = F::cast_from(1.0_f64) / t1099;
    (t1093, t1095, t1098, t1099, t1100)
}
