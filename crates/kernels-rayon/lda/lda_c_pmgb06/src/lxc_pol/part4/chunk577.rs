//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 577/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk577(t2497: f64, t493: f64, t809: f64, t822: f64, t1385: f64) -> (f64, f64, f64) {
    let t2499 = 2.0_f64 / 45.0_f64 * t493 * t2497;
    let t2500 = t809 * t822;
    let t2501 = t1385 * t2500;
    (t2499, t2500, t2501)
}
