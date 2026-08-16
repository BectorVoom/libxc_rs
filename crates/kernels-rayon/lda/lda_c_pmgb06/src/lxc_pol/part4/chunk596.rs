//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 596/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk596(t2574: f64, t473: f64, t2578: f64, t103: f64, t1607: f64, t1614: f64, t1856: f64, t2052: f64, t2572: f64, t2576: f64, t2580: f64, t2639: f64) -> (f64, f64, f64) {
    let t2642 = t473 * t2574;
    let t2645 = t473 * t2578;
    let t2648 = t1607 + 0.023994444444444443_f64 * t1856 - 0.023994444444444443_f64 * t2572 + 0.07198333333333333_f64 * t2576 - 0.035991666666666665_f64 * t2580 + t1614 + 0.008888888888888889_f64 * t2052 - 0.0022222222222222222_f64 * t103 * t2639 + 0.013333333333333334_f64 * t103 * t2642 - 0.006666666666666667_f64 * t103 * t2645;
    (t2642, t2645, t2648)
}
