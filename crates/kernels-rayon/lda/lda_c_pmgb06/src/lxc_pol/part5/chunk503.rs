//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 503/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk503(t2549: f64, t506: f64, t36: f64, t1473: f64, t1818: f64, t2543: f64, t2547: f64) -> (f64, f64, f64) {
    let t2550 = t506 * t2549;
    let t2551 = t36 * t2550;
    let t2553 = -t1473 - 0.0012594444444444445_f64 * t1818 + 0.0012594444444444445_f64 * t2543 - 0.003778333333333333_f64 * t2547 + 0.0018891666666666666_f64 * t2551;
    (t2550, t2551, t2553)
}
