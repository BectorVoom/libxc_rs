//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1079/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1079(t16298: f64, t16305: f64, t16307: f64, t16309: f64, t16314: f64, t188: f64, t539: f64, t7364: f64, t16350: f64, t10687: f64, t10690: f64, t12448: f64, t12450: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19975 = t16298 / 15.0_f64;
    let t19976 = t16305 / 45.0_f64;
    let t19977 = 2.0_f64 / 81.0_f64 * t16307;
    let t19978 = 4.0_f64 / 15.0_f64 * t16309;
    let t19979 = 4.0_f64 / 135.0_f64 * t16314;
    let t19981 = t7364 * t539 * t188;
    let t19983 = 4.0_f64 / 135.0_f64 * t16350;
    let t19984 = -t19975 - t19976 - t10687 + t10690 - t19977 + t19978 - t19979 - t12448 - t12450 + 4.0_f64 / 3.0_f64 * t19981 + t19983;
    (t19975, t19976, t19977, t19978, t19979, t19983, t19984)
}
