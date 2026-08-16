//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1161/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1161(t439: f64, t5482: f64, t6364: f64, t2010: f64, t6371: f64, t1444: f64, t7535: f64, t493: f64, t6751: f64, t6765: f64, t12617: f64, t6769: f64) -> (f64, f64, f64, f64, f64) {
    let t20950 = 2.0_f64 / 15.0_f64 * t439 * t5482 * t6364;
    let t20953 = 4.0_f64 / 15.0_f64 * t2010 * t5482 * t6371;
    let t20955 = 2.0_f64 / 15.0_f64 * t1444 * t7535;
    let t20958 = t493 * t6751 * t6765 / 9.0_f64;
    let t20961 = t493 * t12617 * t6769 / 9.0_f64;
    (t20950, t20953, t20955, t20958, t20961)
}
