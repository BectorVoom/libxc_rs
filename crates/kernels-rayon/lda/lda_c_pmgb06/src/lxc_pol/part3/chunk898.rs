//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 898/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk898(t2943: f64, t517: f64, t132: f64, t3059: f64, t435: f64, t1547: f64, t1595: f64, t1499: f64, t1636: f64, t2880: f64, t486: f64, t161: f64, t3460: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9603 = t2943 * t517;
    let t9616 = t132 * t435 * t3059;
    let t9619 = t132 * t1547 * t1595;
    let t9626 = t1499 * t1636;
    let t9628 = t486 * t2880;
    let t9633 = t161 * t489 * t3460;
    (t9603, t9616, t9619, t9626, t9628, t9633)
}
