//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 426/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk426(t1526: f64, t1619: f64, t1532: f64, t473: f64, t1536: f64, t103: f64, t1523: f64, t1528: f64, t1534: f64, t1538: f64, t1607: f64, t1614: f64, t1615: f64) -> (f64, f64, f64, f64) {
    let t1620 = t1619 * t1526;
    let t1623 = t473 * t1532;
    let t1626 = t473 * t1536;
    let t1629 = t1607 + 0.023994444444444443_f64 * t1523 - 0.023994444444444443_f64 * t1528 + 0.07198333333333333_f64 * t1534 - 0.035991666666666665_f64 * t1538 + t1614 + 0.008888888888888889_f64 * t1615 - 0.0022222222222222222_f64 * t103 * t1620 + 0.013333333333333334_f64 * t103 * t1623 - 0.006666666666666667_f64 * t103 * t1626;
    (t1620, t1623, t1626, t1629)
}
