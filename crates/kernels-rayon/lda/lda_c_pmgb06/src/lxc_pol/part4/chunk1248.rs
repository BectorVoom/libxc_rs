//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1248/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1248(t12644: f64, t12649: f64, t1444: f64, t6533: f64, t531: f64, t6688: f64, t1641: f64, t2563: f64, t1588: f64, t1592: f64, t2582: f64, t132: f64, t137: f64, t1594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16424 = 8.0_f64 / 135.0_f64 * t12644;
    let t16425 = 8.0_f64 / 405.0_f64 * t12649;
    let t16427 = 4.0_f64 / 45.0_f64 * t1444 * t6533;
    let t16429 = t6688 * t531 / 15.0_f64;
    let t16431 = t2563 * t1641 / 15.0_f64;
    let t16433 = t2563 * t1588 / 30.0_f64;
    let t16434 = t2582 * t1592;
    let t16438 = t132 * t137 * t16434 * t1594 / 15.0_f64;
    (t16424, t16425, t16427, t16429, t16431, t16433, t16438)
}
