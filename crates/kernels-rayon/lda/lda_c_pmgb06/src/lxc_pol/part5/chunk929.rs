//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 929/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk929(t1435: f64, t3092: f64, t136: f64, t1438: f64, t3098: f64, t441: f64, t1548: f64, t1887: f64, t2857: f64, t802: f64, t161: f64, t3004: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12397 = t1435 * t3092;
    let t12402 = t136 * t1438;
    let t12406 = t441 * t3098;
    let t12447 = t1887 * t1548;
    let t12448 = t12447 / 45.0_f64;
    let t12449 = t802 * t2857;
    let t12450 = t12449 / 45.0_f64;
    let t12456 = t161 * t3004 * t852;
    (t12397, t12402, t12406, t12448, t12450, t12456)
}
