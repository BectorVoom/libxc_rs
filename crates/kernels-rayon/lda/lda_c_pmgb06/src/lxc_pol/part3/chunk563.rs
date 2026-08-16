//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 563/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk563(t1532: f64, t477: f64, t1385: f64, t439: f64, t1531: f64, t332: f64, t1074: f64) -> (f64, f64, f64, f64, f64) {
    let t2965 = t1532 * t477;
    let t2966 = t1385 * t2965;
    let t2968 = 2.0_f64 / 15.0_f64 * t439 * t2966;
    let t2969 = t1531 * t332;
    let t2970 = t2969 * t1074;
    (t2965, t2966, t2968, t2969, t2970)
}
