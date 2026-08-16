//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1291/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1291(t2466: f64, t3226: f64, t1447: f64, t6541: f64, t6545: f64, t2470: f64, t6282: f64, t13196: f64, t2002: f64, t4609: f64, t13199: f64, t13201: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16962 = t3226 * t2466;
    let t16963 = 4.0_f64 / 135.0_f64 * t16962;
    let t16964 = t1447 * t6541;
    let t16965 = 4.0_f64 / 135.0_f64 * t16964;
    let t16966 = t1447 * t6545;
    let t16967 = 4.0_f64 / 135.0_f64 * t16966;
    let t16968 = t3226 * t2470;
    let t16969 = 4.0_f64 / 81.0_f64 * t16968;
    let t16970 = t1447 * t6282;
    let t16971 = 4.0_f64 / 81.0_f64 * t16970;
    let t16972 = 16.0_f64 / 135.0_f64 * t13196;
    let t16974 = 2.0_f64 / 15.0_f64 * t2002 * t4609;
    let t16975 = 16.0_f64 / 135.0_f64 * t13199;
    let t16976 = 16.0_f64 / 135.0_f64 * t13201;
    (t16963, t16965, t16967, t16969, t16971, t16972, t16974, t16975, t16976)
}
