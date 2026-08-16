//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1282/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1282(t12012: f64, t1924: f64, t493: f64, t497: f64, t6904: f64, t1380: f64, t337: f64, t2002: f64, t5483: f64, t1444: f64, t6791: f64, t9921: f64) -> (f64, f64, f64, f64, f64) {
    let t16855 = 4.0_f64 / 45.0_f64 * t493 * t12012 * t1924;
    let t16856 = t6904 * t497;
    let t16860 = 2.0_f64 / 45.0_f64 * t493 * t1380 * t16856 * t337;
    let t16862 = 4.0_f64 / 45.0_f64 * t2002 * t5483;
    let t16864 = 4.0_f64 / 45.0_f64 * t1444 * t6791;
    let t16865 = 4.0_f64 / 405.0_f64 * t9921;
    (t16855, t16860, t16862, t16864, t16865)
}
