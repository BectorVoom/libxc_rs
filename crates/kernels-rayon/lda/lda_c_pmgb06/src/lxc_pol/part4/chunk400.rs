//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 400/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk400(t1451: f64, t493: f64, t1083: f64, t498: f64, t496: f64) -> (f64, f64, f64) {
    let t1453 = 2.0_f64 / 45.0_f64 * t493 * t1451;
    let t1454 = t498 * t1083;
    let t1455 = t496 * t1454;
    (t1453, t1454, t1455)
}
