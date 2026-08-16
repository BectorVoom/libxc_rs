//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1223/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1223(t1431: f64, t6127: f64, t1441: f64, t1430: f64, t439: f64, t6123: f64, t1435: f64, t2582: f64, t1440: f64, t12041: f64, t1995: f64, t5305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16112 = t6127 * t1431 / 45.0_f64;
    let t16114 = t6127 * t1441 / 27.0_f64;
    let t16117 = t439 * t6123 * t1430 / 45.0_f64;
    let t16118 = t1435 * t2582;
    let t16121 = t439 * t16118 * t1440 / 27.0_f64;
    let t16122 = 16.0_f64 / 1215.0_f64 * t12041;
    let t16124 = 4.0_f64 / 15.0_f64 * t5305 * t1995;
    (t16112, t16114, t16117, t16121, t16122, t16124)
}
