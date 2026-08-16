//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 594/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk594(t1593: f64, t2604: f64, t137: f64, t132: f64, t1576: f64, t2541: f64, t2545: f64, t525: f64, t2549: f64, t103: f64, t1563: f64, t1571: f64, t1818: f64, t2077: f64, t2543: f64, t2547: f64, t2551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2605 = t1593 * t2604;
    let t2606 = t137 * t2605;
    let t2608 = t132 * t2606 / 15.0_f64;
    let t2614 = t1576 * t2541;
    let t2617 = t525 * t2545;
    let t2620 = t525 * t2549;
    let t2623 = t1563 + 0.023994444444444443_f64 * t1818 - 0.023994444444444443_f64 * t2543 + 0.07198333333333333_f64 * t2547 - 0.035991666666666665_f64 * t2551 + t1571 + 0.008888888888888889_f64 * t2077 - 0.0022222222222222222_f64 * t103 * t2614 + 0.013333333333333334_f64 * t103 * t2617 - 0.006666666666666667_f64 * t103 * t2620;
    (t2605, t2606, t2608, t2614, t2617, t2620, t2623)
}
