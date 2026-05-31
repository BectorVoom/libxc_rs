//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1227/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1227<F: Float>(t2493: F, t3220: F, t132: F, t1547: F, t2605: F, t2088: F, t1601: F, t161: F, t166: F, t4839: F, t497: F, t843: F) -> (F, F, F, F) {
    let t16158 = t3220 * t2493;
    let t16159 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t16158;
    let t16161 = t132 * t1547 * t2605;
    let t16162 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t16161;
    let t16163 = t2088 * t2088;
    let t16167 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t161 * t166 * t1601 * t16163;
    let t16171 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t161 * t4839 * t843 * t497;
    (t16159, t16162, t16167, t16171)
}
