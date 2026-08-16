//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1053/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1053(t13026: f64, t13027: f64, t19618: f64, t2377: f64, t822: f64, t477: f64, t12519: f64, t5083: f64, t332: f64, t13031: f64, t1: f64, t6637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19621 = 8.0_f64 / 27.0_f64 * t13026 * t13027 * t19618;
    let t19622 = t2377 * t822;
    let t19623 = t19622 * t477;
    let t19626 = 2.0_f64 / 9.0_f64 * t5083 * t12519 * t19623;
    let t19627 = t19622 * t332;
    let t19630 = 8.0_f64 / 27.0_f64 * t13026 * t13031 * t19627;
    let t19631 = t6637 * t1;
    (t19621, t19623, t19626, t19627, t19630, t19631)
}
