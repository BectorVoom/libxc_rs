//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1259/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1259(t13007: f64, t6630: f64, t1636: f64, t2563: f64, t12752: f64, t12784: f64, t12787: f64, t1593: f64, t2648: f64, t1386: f64, t5077: f64, t15855: f64, t5079: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16556 = t13007 * t6630;
    let t16557 = 16.0_f64 / 135.0_f64 * t16556;
    let t16558 = t2563 * t1636;
    let t16559 = 2.0_f64 / 45.0_f64 * t16558;
    let t16560 = 8.0_f64 / 405.0_f64 * t12752;
    let t16561 = 8.0_f64 / 135.0_f64 * t12784;
    let t16562 = 2.0_f64 / 45.0_f64 * t12787;
    let t16563 = t1593 * t2648;
    let t16566 = 4.0_f64 / 45.0_f64 * t5077 * t16563 * t1386;
    let t16568 = 8.0_f64 / 45.0_f64 * t15855 * t5079;
    (t16557, t16559, t16560, t16561, t16562, t16566, t16568)
}
