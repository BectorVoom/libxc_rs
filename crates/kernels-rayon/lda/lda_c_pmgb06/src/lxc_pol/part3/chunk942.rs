//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 942/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk942(t8483: f64, t8527: f64, t8529: f64, t8536: f64, t8538: f64, t248: f64, t4515: f64, t686: f64, t1069: f64, t395: f64, t247: f64, t332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10997 = 480.0_f64 * t8483;
    let t10999 = 96.0_f64 * t8527;
    let t11000 = 36.0_f64 * t8529;
    let t11002 = 48.0_f64 * t8536;
    let t11003 = 12.0_f64 * t8538;
    let t11007 = t248 * t4515 * t686;
    let t11013 = t395 * t1069;
    let t11021 = t247 * t332;
    (t10997, t10999, t11000, t11002, t11003, t11007, t11013, t11021)
}
