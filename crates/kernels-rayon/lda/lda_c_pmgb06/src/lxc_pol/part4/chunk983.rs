//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 983/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk983(t1090: f64, t1105: f64, t1092: f64, t1101: f64, t3767: f64, t643: f64, t248: f64, t3890: f64, t653: f64, t1024: f64, t3697: f64, t634: f64) -> (f64, f64, f64, f64, f64) {
    let t8541 = t1105 * t1090;
    let t8543 = t1101 * t1092;
    let t8545 = t643 * t3767;
    let t8548 = t248 * t653 * t3890;
    let t8552 = 8.0_f64 * t1024 * t634 * t3697;
    (t8541, t8543, t8545, t8548, t8552)
}
