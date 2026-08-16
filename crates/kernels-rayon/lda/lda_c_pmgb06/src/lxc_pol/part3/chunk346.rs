//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 346/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk346(t109: f64, t342: f64, t55: f64, t1243: f64, t349: f64, t947: f64, t1234: f64, t38: f64, t64: f64, t1227: f64, t56: f64, t409: f64, t54: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1245 = t55 * t109 * t342;
    let t1246 = t1243 * t1245;
    let t1247 = 0.9743416666666667_f64 * t1246;
    let t1249 = 0.6495611111111111_f64 * t349 * t947;
    let t1252 = 5.84605_f64 * t38 * t64 * t1234;
    let t1255 = 2.923025_f64 * t38 * t56 * t1227;
    let t1259 = t54 * t55 * t409 * t56 / 9.0_f64;
    (t1245, t1246, t1247, t1249, t1252, t1255, t1259)
}
