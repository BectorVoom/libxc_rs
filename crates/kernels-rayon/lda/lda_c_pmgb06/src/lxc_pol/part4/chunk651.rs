//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 651/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk651(t144: f64, t3259: f64, t1423: f64, t1431: f64, t1441: f64, t1435: f64, t458: f64, t1592: f64, t1595: f64, t435: f64, t132: f64, t1555: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3260 = t3259 * t144;
    let t3272 = t1423 * t1431;
    let t3274 = t1423 * t1441;
    let t3279 = t1435 * t458;
    let t3290 = t458 * t1592;
    let t3295 = t435 * t1595;
    let t3296 = t132 * t3295;
    let t3306 = t486 * t1555;
    (t3260, t3272, t3274, t3279, t3290, t3295, t3296, t3306)
}
