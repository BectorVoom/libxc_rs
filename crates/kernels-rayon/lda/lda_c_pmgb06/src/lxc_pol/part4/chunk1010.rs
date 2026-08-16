//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1010/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1010(t1387: f64, t3213: f64, t1683: f64, t1730: f64, t1687: f64, t4159: f64, t573: f64, t580: f64, t206: f64, t208: f64, t247: f64, t161: f64, t3004: f64, t512: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9330 = t3213 * t1387;
    let t9338 = t1683 * t1730;
    let t9340 = t1687 * t1730;
    let t9342 = t573 * t4159;
    let t9345 = 0.26596355555555556_f64 * t580 * t4159;
    let t9348 = 0.19208479012345678_f64 * t206 * t247 * t208;
    let t9350 = t161 * t3004 * t512;
    (t9330, t9338, t9340, t9342, t9345, t9348, t9350)
}
