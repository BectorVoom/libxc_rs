//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 894/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk894(t2604: f64, t3290: f64, t137: f64, t132: f64, t2601: f64, t486: f64, t2599: f64, t3038: f64, t166: f64, t161: f64, t1887: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6442 = t3290 * t2604;
    let t6443 = t137 * t6442;
    let t6445 = t132 * t6443 / 15.0_f64;
    let t6447 = t486 * t2601 / 15.0_f64;
    let t6448 = t3038 * t2599;
    let t6449 = t166 * t6448;
    let t6451 = t161 * t6449 / 15.0_f64;
    let t6453 = t1887 * t824 / 15.0_f64;
    (t6442, t6443, t6445, t6447, t6448, t6449, t6451, t6453)
}
