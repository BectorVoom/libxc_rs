//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 847/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk847(t297: f64, t301: f64, t8173: f64, t83: f64, t1193: f64, t398: f64, t4001: f64, t4299: f64, t2841: f64, t4297: f64, t4317: f64, t707: f64) -> (f64, f64, f64, f64) {
    let t8177 = 0.01197423401025461_f64 * t297 * t83 * t8173 * t301;
    let t8180 = t4001 * t398 * t1193 * t4299;
    let t8184 = 1.8276876377896586e-05_f64 * t4297 * t2841 * t4299;
    let t8189 = t707 * t4317;
    (t8177, t8180, t8184, t8189)
}
