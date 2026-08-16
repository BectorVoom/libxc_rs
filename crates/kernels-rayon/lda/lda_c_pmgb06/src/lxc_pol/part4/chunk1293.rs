//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1293/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1293(t2466: f64, t3198: f64, t13204: f64, t13206: f64, t1444: f64, t6541: f64, t6545: f64, t2465: f64, t3194: f64, t493: f64, t1450: f64, t6544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16979 = t3198 * t2466 / 45.0_f64;
    let t16980 = 4.0_f64 / 135.0_f64 * t13204;
    let t16981 = 8.0_f64 / 135.0_f64 * t13206;
    let t16983 = 2.0_f64 / 45.0_f64 * t1444 * t6541;
    let t16985 = 2.0_f64 / 45.0_f64 * t1444 * t6545;
    let t16988 = t493 * t3194 * t2465 / 45.0_f64;
    let t16991 = 2.0_f64 / 45.0_f64 * t493 * t1450 * t6544;
    (t16979, t16980, t16981, t16983, t16985, t16988, t16991)
}
