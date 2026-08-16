//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1245/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1245(t1894: f64, t5187: f64, t2002: f64, t5287: f64, t5291: f64, t5295: f64, t6275: f64, t1898: f64, t1925: f64, t5305: f64, t1972: f64, t5477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16396 = 4.0_f64 / 45.0_f64 * t5187 * t1894;
    let t16398 = 4.0_f64 / 45.0_f64 * t2002 * t5287;
    let t16400 = 2.0_f64 / 45.0_f64 * t2002 * t5291;
    let t16402 = 8.0_f64 / 45.0_f64 * t6275 * t5295;
    let t16404 = 8.0_f64 / 45.0_f64 * t5187 * t1898;
    let t16406 = 4.0_f64 / 45.0_f64 * t5305 * t1925;
    let t16408 = 4.0_f64 / 45.0_f64 * t1972 * t5477;
    (t16396, t16398, t16400, t16402, t16404, t16406, t16408)
}
