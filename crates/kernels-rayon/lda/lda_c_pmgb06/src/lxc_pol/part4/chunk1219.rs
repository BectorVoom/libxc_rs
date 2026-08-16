//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1219/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1219(t1444: f64, t6403: f64, t493: f64, t5447: f64, t6402: f64, t1083: f64, t2541: f64, t1915: f64, t9402: f64, t16040: f64, t16044: f64, t16048: f64, t16050: f64, t16052: f64, t16054: f64, t16056: f64, t16058: f64, t16060: f64, t16063: f64, t16067: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16069 = 4.0_f64 / 15.0_f64 * t1444 * t6403;
    let t16072 = 4.0_f64 / 15.0_f64 * t493 * t5447 * t6402;
    let t16073 = t2541 * t1083;
    let t16076 = 2.0_f64 / 15.0_f64 * t493 * t1915 * t16073;
    let t16077 = t9402 / 135.0_f64;
    let t16078 = -t16040 - t16044 - t16048 - t16050 + t16052 + t16054 - t16056 - t16058 + t16060 + t16063 + t16067 + t16069 + t16072 + t16076 - t16077;
    (t16069, t16072, t16073, t16076, t16077, t16078)
}
