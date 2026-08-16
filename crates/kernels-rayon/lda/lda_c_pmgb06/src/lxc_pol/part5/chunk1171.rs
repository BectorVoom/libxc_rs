//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1171/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1171(t17886: f64, t17890: f64, t1444: f64, t7715: f64, t2979: f64, t493: f64, t7714: f64, t1380: f64, t2088: f64, t2545: f64, t1423: f64, t7525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21068 = 8.0_f64 / 45.0_f64 * t17886;
    let t21069 = 4.0_f64 / 27.0_f64 * t17890;
    let t21071 = 2.0_f64 / 15.0_f64 * t1444 * t7715;
    let t21074 = 2.0_f64 / 15.0_f64 * t493 * t2979 * t7714;
    let t21078 = 2.0_f64 / 15.0_f64 * t493 * t1380 * t2545 * t2088;
    let t21079 = t1423 * t7525;
    (t21068, t21069, t21071, t21074, t21078, t21079)
}
