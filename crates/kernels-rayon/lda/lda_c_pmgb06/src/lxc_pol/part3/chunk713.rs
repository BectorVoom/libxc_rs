//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 713/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk713(t1869: f64, t4641: f64, t3092: f64, t760: f64, t1069: f64, t3090: f64, t36: f64, t3098: f64, t1525: f64, t1: f64, t1438: f64, t332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4642 = t4641 * t1869;
    let t4644 = t3092 * t760;
    let t4645 = t4644 * t1069;
    let t4646 = t3090 * t4645;
    let t4647 = t36 * t4646;
    let t4649 = t3098 * t760;
    let t4650 = t4649 * t1069;
    let t4651 = t1525 * t4650;
    let t4652 = t36 * t4651;
    let t4654 = t1438 * t1;
    let t4655 = t4654 * t332;
    (t4642, t4644, t4645, t4646, t4647, t4649, t4650, t4651, t4652, t4654, t4655)
}
