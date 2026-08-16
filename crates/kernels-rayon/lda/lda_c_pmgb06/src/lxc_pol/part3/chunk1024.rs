//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1024/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1024(t439: f64, t4650: f64, t5253: f64, t3010: f64, t760: f64, t9220: f64, t5260: f64, t1: f64, t1069: f64, t3098: f64, t1901: f64, t2010: f64) -> (f64, f64, f64, f64, f64) {
    let t12174 = 2.0_f64 / 3.0_f64 * t439 * t5253 * t4650;
    let t12176 = t9220 * t760 * t3010;
    let t12179 = 32.0_f64 / 27.0_f64 * t439 * t5260 * t12176;
    let t12181 = t3098 * t1 * t1069;
    let t12184 = 4.0_f64 / 3.0_f64 * t2010 * t1901 * t12181;
    (t12174, t12176, t12179, t12181, t12184)
}
