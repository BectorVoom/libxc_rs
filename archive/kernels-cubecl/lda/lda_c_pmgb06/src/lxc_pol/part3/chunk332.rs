//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 332/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk332<F: Float>(t1156: F, t123: F, t199: F, t566: F, t722: F, t81: F, t1072: F, t1105: F) -> (F, F, F, F, F) {
    let t1158 = t123 * t1156 * t199;
    let t1161 = t123 * t722 * t566;
    let t1163 = F::cast_from(2.0_f64) * t81;
    let t1164 = F::cast_from(8.0_f64) * t1072;
    let t1165 = F::cast_from(6.0_f64) * t1105;
    let t1166 = -t1163 + t1164 - t1165;
    (t1158, t1161, t1163, t1165, t1166)
}
