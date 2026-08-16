//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 881/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk881(t2012: f64, t6275: f64, t1420: f64, t2477: f64, t1444: f64, t2470: f64, t2469: f64, t3238: f64, t493: f64, t2599: f64, t3457: f64, t529: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6277 = 4.0_f64 / 45.0_f64 * t6275 * t2012;
    let t6279 = 2.0_f64 / 45.0_f64 * t1420 * t2477;
    let t6281 = t1444 * t2470 / 27.0_f64;
    let t6282 = t3238 * t2469;
    let t6284 = t493 * t6282 / 27.0_f64;
    let t6285 = t3457 * t2599;
    let t6286 = t6285 * t529;
    (t6277, t6279, t6281, t6282, t6284, t6285, t6286)
}
