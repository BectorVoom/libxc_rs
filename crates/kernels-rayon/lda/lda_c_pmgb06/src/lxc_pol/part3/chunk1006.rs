//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1006/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1006(t9424: f64, t443: f64, t464: f64, t2010: f64, t442: f64, t477: f64, t1423: f64, t5291: f64, t1420: f64, t5365: f64, t10288: f64, t439: f64, t5364: f64) -> (f64, f64, f64, f64, f64) {
    let t11964 = (2e-21_f64 as f64) * t9424;
    let t11966 = t464 * t443;
    let t11970 = 2.0_f64 / 15.0_f64 * t2010 * t442 * t11966 * t477;
    let t11971 = t1423 * t5291;
    let t11972 = 2.0_f64 / 45.0_f64 * t11971;
    let t11974 = 2.0_f64 / 15.0_f64 * t1420 * t5365;
    let t11977 = 2.0_f64 / 15.0_f64 * t439 * t10288 * t5364;
    (t11964, t11970, t11972, t11974, t11977)
}
