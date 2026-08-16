//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1093/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1093(t16556: f64, t2386: f64, t851: f64, t529: f64, t13064: f64, t5138: f64, t337: f64, t12529: f64, t12530: f64, t1: f64, t6560: f64, t12537: f64, t5139: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20146 = 8.0_f64 / 45.0_f64 * t16556;
    let t20147 = t2386 * t851;
    let t20148 = t20147 * t529;
    let t20151 = 2.0_f64 / 9.0_f64 * t5138 * t13064 * t20148;
    let t20152 = t20147 * t337;
    let t20155 = 8.0_f64 / 27.0_f64 * t12529 * t12530 * t20152;
    let t20156 = t6560 * t1;
    let t20159 = 4.0_f64 / 9.0_f64 * t12537 * t5139 * t20156;
    (t20146, t20148, t20151, t20152, t20155, t20156, t20159)
}
