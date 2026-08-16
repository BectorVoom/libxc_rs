//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 325/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk325(t1238: f64, t39: f64, t955: f64, t27: f64, t348: f64, t56: f64, t109: f64, t342: f64, t55: f64, t349: f64, t947: f64, t409: f64, t54: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1239 = t1238 * t39;
    let t1241 = 0.3247805555555556_f64 * t1239 * t955;
    let t1243 = t348 * t56 * t27;
    let t1245 = t55 * t109 * t342;
    let t1246 = t1243 * t1245;
    let t1249 = 0.6495611111111111_f64 * t349 * t947;
    let t1259 = t54 * t55 * t409 * t56 / 9.0_f64;
    (t1239, t1241, t1243, t1245, t1246, t1249, t1259)
}
