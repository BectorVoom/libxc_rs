//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1048/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1048(t4481: f64, t643: f64, t4516: f64, t638: f64, t1101: f64, t2160: f64, t2158: f64, t2799: f64, t898: f64, t2801: f64, t3947: f64, t3952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11065 = t643 * t4481;
    let t11067 = t638 * t4516;
    let t11070 = t643 * t4516;
    let t11090 = t1101 * t2160;
    let t11092 = t1101 * t2158;
    let t11095 = t2799 * t898;
    let t11097 = t2801 * t898;
    let t11099 = t3947 * t898;
    let t11101 = t3952 * t898;
    (t11065, t11067, t11070, t11090, t11092, t11095, t11097, t11099, t11101)
}
