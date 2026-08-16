//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 800/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk800(t153: f64, t7501: f64, t137: f64, t132: f64, t6423: f64, t6425: f64, t2549: f64, t851: f64, t1380: f64, t493: f64, t6759: f64, t764: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7502 = t7501 * t153;
    let t7503 = t137 * t7502;
    let t7505 = t132 * t7503 / 30.0_f64;
    let t7506 = 4.0_f64 / 45.0_f64 * t6423;
    let t7507 = 4.0_f64 / 45.0_f64 * t6425;
    let t7508 = t2549 * t851;
    let t7509 = t1380 * t7508;
    let t7511 = t493 * t7509 / 15.0_f64;
    let t7512 = t6759 * t764;
    (t7502, t7503, t7505, t7506, t7507, t7508, t7509, t7511, t7512)
}
