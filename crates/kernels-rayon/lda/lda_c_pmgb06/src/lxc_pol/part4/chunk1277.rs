//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1277/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1277(t495: f64, t6831: f64, t493: f64, t499: f64, t132: f64, t1547: f64, t2649: f64, t13087: f64, t12535: f64, t13027: f64, t15324: f64, t3259: f64, t5075: f64) -> (f64, f64, f64, f64) {
    let t16794 = t495 * t6831;
    let t16797 = 2.0_f64 / 45.0_f64 * t493 * t16794 * t499;
    let t16799 = t132 * t1547 * t2649;
    let t16800 = t16799 / 135.0_f64;
    let t16801 = 4.0_f64 / 135.0_f64 * t13087;
    let t16806 = 64.0_f64 / 81.0_f64 * t5075 * t12535 * t3259 * t13027 * t15324;
    (t16797, t16800, t16801, t16806)
}
