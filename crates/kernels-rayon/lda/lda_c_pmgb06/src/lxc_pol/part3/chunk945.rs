//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 945/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk945(t11034: f64, t11060: f64, t257: f64, t4481: f64, t643: f64, t4516: f64, t638: f64, t10997: f64, t10999: f64, t11000: f64, t11002: f64, t11003: f64, t11007: f64, t248: f64, t285: f64, t8482: f64, t8519: f64, t8520: f64, t8526: f64, t8534: f64, t8541: f64, t8543: f64) -> (f64, f64) {
    let t11062 = (t11034 + t11060) * t257;
    let t11065 = t643 * t4481;
    let t11066 = 24.0_f64 * t11065;
    let t11067 = t638 * t4516;
    let t11069 = t248 * t11062 * t285 + t10997 - t10999 - t11000 - t11002 + t11003 + 3.0_f64 * t11007 - t11066 + 12.0_f64 * t11067 + t8482 - t8519 - 360.0_f64 * t8520 + t8526 + 3.0_f64 * t8534 - 36.0_f64 * t8541 + 180.0_f64 * t8543;
    (t11062, t11069)
}
