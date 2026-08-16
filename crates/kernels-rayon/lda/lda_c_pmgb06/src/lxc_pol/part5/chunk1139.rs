//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1139/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1139(t1423: f64, t7651: f64, t2493: f64, t5220: f64, t10134: f64, t13292: f64, t13295: f64, t20663: f64, t20666: f64, t20667: f64, t20668: f64, t20670: f64, t20671: f64, t20673: f64) -> (f64, f64, f64) {
    let t20674 = t1423 * t7651;
    let t20675 = 4.0_f64 / 45.0_f64 * t20674;
    let t20676 = t5220 * t2493;
    let t20677 = 4.0_f64 / 45.0_f64 * t20676;
    let t20678 = -t20663 + t20666 - t13292 - t13295 - t20667 - t20668 + t20670 - t20671 - t10134 + t20673 - t20675 - t20677;
    (t20675, t20677, t20678)
}
