//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 783/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk783(t2026: f64, t591: f64, t1680: f64, t872: f64, t1696: f64, t794: f64, t208: f64, t213: f64, t2025: f64, t97: f64, t588: f64, t205: f64, t4463: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5370 = t2026 * t591;
    let t5372 = t872 * t1680;
    let t5374 = t794 * t1696;
    let t5375 = t5374 * t208;
    let t5376 = t5375 * t213;
    let t5378 = t2025 * t97;
    let t5379 = t5378 * t588;
    let t5381 = t4463 * t205;
    (t5370, t5372, t5374, t5375, t5376, t5378, t5379, t5381)
}
