//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 878/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk878(t4159: f64, t573: f64, t580: f64, t206: f64, t208: f64, t247: f64, t161: f64, t3004: f64, t512: f64, t3005: f64, t486: f64, t1767: f64, t4068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9342 = t573 * t4159;
    let t9345 = 0.26596355555555556_f64 * t580 * t4159;
    let t9348 = 0.19208479012345678_f64 * t206 * t247 * t208;
    let t9350 = t161 * t3004 * t512;
    let t9352 = t486 * t3005;
    let t9408 = 0.008082336938271605_f64 * t206 * t1767 * t4068;
    (t9342, t9345, t9348, t9350, t9352, t9408)
}
