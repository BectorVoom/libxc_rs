//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1159/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1159(t17801: f64, t17809: f64, t2002: f64, t6775: f64, t2979: f64, t493: f64, t7538: f64, t1380: f64, t16856: f64, t764: f64, t1444: f64, t7539: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20919 = 4.0_f64 / 45.0_f64 * t17801;
    let t20920 = 4.0_f64 / 27.0_f64 * t17809;
    let t20922 = t2002 * t6775 / 15.0_f64;
    let t20925 = t493 * t2979 * t7538 / 15.0_f64;
    let t20929 = t493 * t1380 * t16856 * t764 / 15.0_f64;
    let t20931 = t1444 * t7539 / 15.0_f64;
    (t20919, t20920, t20922, t20925, t20929, t20931)
}
