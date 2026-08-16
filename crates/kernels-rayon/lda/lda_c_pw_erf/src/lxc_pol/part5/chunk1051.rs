//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1051/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1051(t646: f64, t7045: f64, t1410: f64, t2463: f64, t656: f64, t6881: f64, t6884: f64, t153: f64, t474: f64, t6080: f64, t1210: f64, t168: f64, t2581: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19249 = t7045 * t646;
    let t19256 = t2463 * t1410;
    let t19318 = t6881 * t656;
    let t19320 = t6884 * t656;
    let t19344 = t153 * t474 * t6080;
    let t19347 = t168 * t1210 * t2581;
    (t19249, t19256, t19318, t19320, t19344, t19347)
}
