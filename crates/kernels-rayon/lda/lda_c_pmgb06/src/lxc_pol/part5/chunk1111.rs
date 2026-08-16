//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1111/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1111(t17964: f64, t1992: f64, t493: f64, t851: f64, t1972: f64, t6287: f64, t2088: f64, t6112: f64, t1444: f64, t7685: f64, t5179: f64, t7684: f64) -> (f64, f64, f64, f64, f64) {
    let t20353 = t493 * t1992 * t17964 * t851 / 5.0_f64;
    let t20355 = 3.0_f64 / 5.0_f64 * t1972 * t6287;
    let t20359 = t493 * t1992 * t6112 * t2088 / 5.0_f64;
    let t20361 = t1444 * t7685 / 5.0_f64;
    let t20364 = t493 * t5179 * t7684 / 5.0_f64;
    (t20353, t20355, t20359, t20361, t20364)
}
