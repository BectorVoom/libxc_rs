//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1044/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1044(t1100: f64, t290: f64, t123: f64, t199: f64, t4297: f64, t1126: f64, t247: f64, t642: f64, t701: f64, t10797: f64, t2833: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10895 = 6.399008129061525_f64 * t1100 * t290;
    let t10902 = 2.4210827305188265_f64 * t123 * t4297 * t199;
    let t10903 = t247 * t1126;
    let t10905 = t642 * t701;
    let t10934 = t123 * t10797 * t199;
    let t10937 = t123 * t2833 * t566;
    (t10895, t10902, t10903, t10905, t10934, t10937)
}
