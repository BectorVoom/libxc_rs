//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1075/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1075(t464: f64, t4779: f64, t1386: f64, t439: f64, t1924: f64, t493: f64, t9925: f64, t1385: f64, t332: f64, t443: f64, t5039: f64, t1387: f64, t5220: f64) -> (f64, f64, f64, f64) {
    let t12772 = t4779 * t464;
    let t12775 = 2.0_f64 / 15.0_f64 * t439 * t12772 * t1386;
    let t12778 = t493 * t9925 * t1924 / 15.0_f64;
    let t12783 = t439 * t1385 * t5039 * t443 * t332 / 15.0_f64;
    let t12784 = t5220 * t1387;
    (t12775, t12778, t12783, t12784)
}
