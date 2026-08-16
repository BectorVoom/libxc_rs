//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1013/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1013(t1415: f64, t1981: f64, t337: f64, t496: f64, t1462: f64, t1465: f64, t1988: f64, t3242: f64, t493: f64, t1992: f64, t1993: f64, t3382: f64) -> (f64, f64, f64, f64) {
    let t12051 = 4.0_f64 / 15.0_f64 * t1981 * t496 * t1415 * t337;
    let t12055 = 2.0_f64 / 9.0_f64 * t1981 * t1462 * t1465 * t337;
    let t12058 = t493 * t1988 * t3242 / 45.0_f64;
    let t12062 = t493 * t1992 * t1993 * t3382 / 15.0_f64;
    (t12051, t12055, t12058, t12062)
}
