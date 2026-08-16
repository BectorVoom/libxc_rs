//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1002/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1002(t432: f64, t5051: f64, t132: f64, t1547: f64, t1873: f64, t5385: f64, t588: f64, t97: f64, t208: f64, t213: f64, t4463: f64, t579: f64) -> (f64, f64, f64, f64) {
    let t11914 = t432 * t5051;
    let t11915 = t11914 / 45.0_f64;
    let t11917 = t132 * t1547 * t1873;
    let t11918 = t11917 / 45.0_f64;
    let t11920 = t5385 * t97 * t588;
    let t11921 = 0.36466666666666664_f64 * t11920;
    let t11928 = t4463 * t579 * t208 * t213;
    (t11915, t11918, t11921, t11928)
}
