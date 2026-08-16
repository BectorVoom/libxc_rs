//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 974/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk974(t11391: f64, t11401: f64, t11403: f64, t11413: f64, t11426: f64, t11427: f64, t11430: f64, t11431: f64, t11436: f64, t11437: f64, t11441: f64, t11443: f64, t11444: f64, t8339: f64) -> f64 {
    let t11525 = t11391 - t11401 - t11403 + t11413 + t11426 - t11427 - t11430 + t11431 + t11436 - t8339 + t11437 + t11441 + t11443 - t11444;
    t11525
}
