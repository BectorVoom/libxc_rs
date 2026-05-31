//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1103/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1103<F: Float>(t4602: F, t5322: F, t1981: F, t1982: F, t3194: F, t1592: F, t1966: F, t439: F, t477: F, t5039: F, t9936: F, t161: F, t166: F, t851: F, t9603: F) -> (F, F, F, F, F) {
    let t13125 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t4602 * t5322;
    let t13128 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1981 * t3194 * t1982;
    let t13133 = t439 * t1966 * t1592 * t5039 * t477 / F::cast_from(5.0_f64);
    let t13134 = t9936 / F::cast_from(45.0_f64);
    let t13138 = t161 * t166 * t9603 * t851 / F::cast_from(30.0_f64);
    (t13125, t13128, t13133, t13134, t13138)
}
