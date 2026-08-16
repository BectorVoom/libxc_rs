//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 882/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk882(t1992: f64, t6286: f64, t493: f64, t1444: f64, t2489: f64, t1450: f64, t2488: f64, t1420: f64, t2493: f64, t1426: f64, t2492: f64, t439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6287 = t1992 * t6286;
    let t6289 = t493 * t6287 / 5.0_f64;
    let t6291 = 2.0_f64 / 45.0_f64 * t1444 * t2489;
    let t6292 = t1450 * t2488;
    let t6294 = 2.0_f64 / 45.0_f64 * t493 * t6292;
    let t6296 = 2.0_f64 / 45.0_f64 * t1420 * t2493;
    let t6297 = t1426 * t2492;
    let t6299 = 2.0_f64 / 45.0_f64 * t439 * t6297;
    (t6287, t6289, t6291, t6292, t6294, t6296, t6297, t6299)
}
