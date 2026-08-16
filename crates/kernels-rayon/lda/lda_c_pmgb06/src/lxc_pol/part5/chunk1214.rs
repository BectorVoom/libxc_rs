//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1214/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1214(t11861: f64, t11867: f64, t19434: f64, t19436: f64, t19438: f64, t19440: f64, t19441: f64, t19442: f64, t9340: f64, t9342: f64, t9345: f64, t9348: f64) -> f64 {
    let t21904 = -t19434 + t19436 + t19438 + t19440 + 0.09973633333333333_f64 * t9340 - 0.06649088888888889_f64 * t9342 - t9345 + t9348 + t19441 - t11861 - t19442 - t11867;
    t21904
}
