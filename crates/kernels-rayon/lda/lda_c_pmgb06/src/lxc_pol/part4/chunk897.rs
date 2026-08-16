//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 897/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk897(t1901: f64, t6146: f64, t439: f64, t5260: f64, t6151: f64, t6155: f64, t2010: f64, t1916: f64, t1972: f64, t1920: f64, t1894: f64, t2002: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6472 = t1901 * t6146;
    let t6474 = 2.0_f64 / 9.0_f64 * t439 * t6472;
    let t6475 = t5260 * t6151;
    let t6477 = 8.0_f64 / 81.0_f64 * t439 * t6475;
    let t6478 = t1901 * t6155;
    let t6480 = 4.0_f64 / 27.0_f64 * t2010 * t6478;
    let t6482 = 4.0_f64 / 45.0_f64 * t1972 * t1916;
    let t6484 = 2.0_f64 / 27.0_f64 * t1972 * t1920;
    let t6486 = 2.0_f64 / 45.0_f64 * t2002 * t1894;
    (t6472, t6474, t6475, t6477, t6478, t6480, t6482, t6484, t6486)
}
