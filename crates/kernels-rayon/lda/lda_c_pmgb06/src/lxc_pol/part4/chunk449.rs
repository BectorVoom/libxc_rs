//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 449/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk449(t707: f64, t711: f64, t715: f64, t113: f64, t1166: f64, t301: f64, t398: f64, t413: f64, t297: f64, t1183: f64, t83: f64, t246: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1750 = t707 * t711;
    let t1753 = 0.039914113367515366_f64 * t707 * t715;
    let t1755 = t1166 * t113 * t301;
    let t1759 = t398 * t413 * t301;
    let t1760 = t297 * t1759;
    let t1763 = t83 * t1183 * t301;
    let t1765 = 0.01197423401025461_f64 * t297 * t1763;
    let t1767 = 1.0_f64 / t33 / t246;
    (t1750, t1753, t1755, t1759, t1760, t1763, t1765, t1767)
}
