//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 974/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk974(t132: f64, t435: f64, t6583: f64, t6571: f64, t1894: f64, t5220: f64, t1898: f64, t1902: f64, t5211: f64, t6478: f64, t432: f64, t6836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16181 = t132 * t435 * t6583;
    let t16184 = t132 * t435 * t6571;
    let t16213 = t5220 * t1894;
    let t16215 = t5220 * t1898;
    let t16217 = t5220 * t1902;
    let t16219 = t5211 * t6478;
    let t16238 = t432 * t6836;
    (t16181, t16184, t16213, t16215, t16217, t16219, t16238)
}
