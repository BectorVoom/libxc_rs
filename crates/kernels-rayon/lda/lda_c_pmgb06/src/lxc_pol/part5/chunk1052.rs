//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1052/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1052(t2381: f64, t822: f64, t477: f64, t5077: f64, t6636: f64, t332: f64, t5094: f64, t7458: f64) -> (f64, f64, f64, f64) {
    let t19609 = t2381 * t822;
    let t19613 = 2.0_f64 / 15.0_f64 * t5077 * t6636 * t19609 * t477;
    let t19614 = t19609 * t332;
    let t19617 = 2.0_f64 / 15.0_f64 * t5077 * t5094 * t19614;
    let t19618 = t7458 * t332;
    (t19613, t19614, t19617, t19618)
}
