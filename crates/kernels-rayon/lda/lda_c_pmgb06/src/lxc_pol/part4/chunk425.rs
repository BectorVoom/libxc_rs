//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 425/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk425(t1477: f64, t1576: f64, t1481: f64, t525: f64, t1485: f64, t103: f64, t1474: f64, t1479: f64, t1483: f64, t1487: f64, t1563: f64, t1571: f64, t1572: f64) -> (f64, f64, f64, f64) {
    let t1577 = t1576 * t1477;
    let t1580 = t525 * t1481;
    let t1583 = t525 * t1485;
    let t1586 = t1563 + 0.023994444444444443_f64 * t1474 - 0.023994444444444443_f64 * t1479 + 0.07198333333333333_f64 * t1483 - 0.035991666666666665_f64 * t1487 + t1571 + 0.008888888888888889_f64 * t1572 - 0.0022222222222222222_f64 * t103 * t1577 + 0.013333333333333334_f64 * t103 * t1580 - 0.006666666666666667_f64 * t103 * t1583;
    (t1577, t1580, t1583, t1586)
}
