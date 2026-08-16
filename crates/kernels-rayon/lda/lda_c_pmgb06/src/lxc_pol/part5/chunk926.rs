//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 926/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk926(t12112: f64, t10203: f64, t153: f64, t1680: f64, t2022: f64, t2026: f64, t132: f64, t2851: f64, t814: f64, t2852: f64, t802: f64, t1554: f64, t161: f64, t2100: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12113 = t12112 / 45.0_f64;
    let t12154 = t10203 * t153;
    let t12224 = t2022 * t1680;
    let t12225 = 2.0_f64 / 9.0_f64 * t12224;
    let t12227 = t2026 * t1680;
    let t12232 = t132 * t2851 * t814;
    let t12234 = t802 * t2852;
    let t12239 = t161 * t1554 * t2100;
    (t12113, t12154, t12225, t12227, t12232, t12234, t12239)
}
