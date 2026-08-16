//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1132/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1132(t20569: f64, t20572: f64, t20575: f64, t20577: f64, t20579: f64, t20581: f64, t20584: f64, t20587: f64, t20589: f64, t20592: f64, t20596: f64, t1380: f64, t2088: f64, t2549: f64, t493: f64) -> (f64, f64) {
    let t20597 = t20569 + t20572 - t20575 + t20577 + t20579 - t20581 - t20584 + t20587 - t20589 - t20592 - t20596;
    let t20601 = t493 * t1380 * t2549 * t2088 / 15.0_f64;
    (t20597, t20601)
}
