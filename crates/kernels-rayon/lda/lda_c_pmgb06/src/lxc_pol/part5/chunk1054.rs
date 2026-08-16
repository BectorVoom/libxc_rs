//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1054/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1054(t13020: f64, t19631: f64, t5084: f64, t497: f64, t7857: f64, t1380: f64, t337: f64, t493: f64, t19599: f64, t19602: f64, t19605: f64, t19608: f64, t19613: f64, t19617: f64, t19621: f64, t19626: f64, t19630: f64) -> (f64, f64, f64) {
    let t19634 = 4.0_f64 / 9.0_f64 * t13020 * t5084 * t19631;
    let t19635 = t7857 * t497;
    let t19639 = t493 * t1380 * t19635 * t337 / 45.0_f64;
    let t19640 = t19599 - t19602 + t19605 - t19608 + t19613 + t19617 + t19621 + t19626 - t19630 - t19634 - t19639;
    (t19634, t19639, t19640)
}
