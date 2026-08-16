//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 349/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk349(t1245: f64, t1276: f64, t366: f64, t947: f64, t18: f64, t369: f64) -> (f64, f64, f64) {
    let t1277 = t1276 * t1245;
    let t1280 = 0.3264533333333333_f64 * t366 * t947;
    let t1282 = 1.0_f64 / t369 / t18;
    (t1277, t1280, t1282)
}
