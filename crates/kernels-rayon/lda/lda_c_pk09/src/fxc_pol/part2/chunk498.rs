//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 498/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk498(t2052: f64, t2795: f64, t2730: f64, t55: f64, t285: f64, t1759: f64, t2149: f64, t54: f64, t433: f64, t2008: f64, t2010: f64, t2012: f64, t2014: f64, t2733: f64, t2736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2796 = t2795 * t2052;
    let t2801 = t55 * t2730;
    let t2802 = t285 * t2801;
    let t2803 = t1759 * t2802;
    let t2805 = t54 * t2149;
    let t2806 = t285 * t2805;
    let t2807 = t433 * t2806;
    let t2811 = t2008 - 0.22687409291590604_f64 * t2803 + t2010 + 0.22687409291590604_f64 * t2807 + t2012 - 0.04525483399593904_f64 * t2733 + t2014 + 0.04525483399593904_f64 * t2736;
    (t2796, t2801, t2803, t2805, t2807, t2811)
}
