//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1162/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1162(t2979: f64, t4731: f64, t493: f64, t1981: f64, t5441: f64, t1380: f64, t3382: f64, t838: f64, t1912: f64, t3226: f64, t1447: f64, t4728: f64) -> (f64, f64, f64, f64, f64) {
    let t13875 = t493 * t2979 * t4731 / 15.0_f64;
    let t13878 = 4.0_f64 / 15.0_f64 * t1981 * t2979 * t5441;
    let t13882 = t493 * t1380 * t838 * t3382 / 45.0_f64;
    let t13883 = t3226 * t1912;
    let t13884 = 4.0_f64 / 45.0_f64 * t13883;
    let t13885 = t1447 * t4728;
    (t13875, t13878, t13882, t13884, t13885)
}
