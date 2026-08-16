//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1233/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1233(t16241: f64, t12232: f64, t12234: f64, t161: f64, t166: f64, t2093: f64, t4935: f64, t1447: f64, t6114: f64, t1972: f64, t5319: f64, t14347: f64, t14350: f64, t14353: f64, t14356: f64, t14359: f64, t16228: f64, t16237: f64, t16239: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16242 = 2.0_f64 / 45.0_f64 * t16241;
    let t16243 = 8.0_f64 / 405.0_f64 * t12232;
    let t16244 = 8.0_f64 / 405.0_f64 * t12234;
    let t16248 = t161 * t166 * t2093 * t4935 / 15.0_f64;
    let t16249 = t1447 * t6114;
    let t16250 = 4.0_f64 / 45.0_f64 * t16249;
    let t16252 = 2.0_f64 / 15.0_f64 * t1972 * t5319;
    let t16253 = t16228 + 0.04472697096444135_f64 * t14347 + 0.06709045644666203_f64 * t14350 + 0.21642082724729686_f64 * t14353 + 0.8656833089891874_f64 * t14356 + 0.6492624817418906_f64 * t14359 + t16237 + t16239 + t16242 + t16243 + t16244 - t16248 + t16250 + t16252;
    (t16242, t16243, t16244, t16248, t16250, t16252, t16253)
}
