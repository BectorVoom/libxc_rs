//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 904/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk904(t123: f64, t199: f64, t4297: f64, t642: f64, t701: f64, t10793: f64, t2822: f64, t566: f64, t247: f64, t4344: f64, t749: f64, t327: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10902 = 2.4210827305188265_f64 * t123 * t4297 * t199;
    let t10905 = t642 * t701;
    let t10943 = t123 * t10793 * t199;
    let t10946 = t123 * t2822 * t566;
    let t10967 = t247 * t749 * t4344;
    let t10970 = t327 * t327;
    (t10902, t10905, t10943, t10946, t10967, t10970)
}
