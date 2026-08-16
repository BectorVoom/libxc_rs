//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1154/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1154(t1444: f64, t7640: f64, t1450: f64, t493: f64, t7639: f64, t19395: f64, t496: f64, t498: f64, t2470: f64, t5305: f64, t1972: f64, t6282: f64) -> (f64, f64, f64, f64, f64) {
    let t20863 = t1444 * t7640 / 45.0_f64;
    let t20866 = t493 * t1450 * t7639 / 45.0_f64;
    let t20870 = t493 * t496 * t498 * t19395 / 45.0_f64;
    let t20872 = t5305 * t2470 / 9.0_f64;
    let t20874 = t1972 * t6282 / 9.0_f64;
    (t20863, t20866, t20870, t20872, t20874)
}
