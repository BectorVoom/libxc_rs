//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 352/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk352(t109: f64, t342: f64, t55: f64, t1243: f64, t349: f64, t947: f64) -> (f64, f64, f64, f64) {
    let t1245 = t55 * t109 * t342;
    let t1246 = t1243 * t1245;
    let t1247 = 0.9743416666666667_f64 * t1246;
    let t1249 = 0.6495611111111111_f64 * t349 * t947;
    (t1245, t1246, t1247, t1249)
}
