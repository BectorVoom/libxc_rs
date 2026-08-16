//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1341/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1341(t1386: f64, t17617: f64, t5077: f64, t2599: f64, t3458: f64, t1381: f64, t5068: f64, t5090: f64, t5493: f64, t2604: f64, t3032: f64, t5078: f64, t5232: f64) -> (f64, f64, f64, f64, f64) {
    let t17620 = 8.0_f64 / 45.0_f64 * t5077 * t17617 * t1386;
    let t17621 = t3458 * t2599;
    let t17624 = 4.0_f64 / 15.0_f64 * t5068 * t17621 * t1381;
    let t17627 = 8.0_f64 / 45.0_f64 * t5068 * t5090 * t5493;
    let t17628 = t3032 * t2604;
    let t17631 = 4.0_f64 / 15.0_f64 * t5077 * t17628 * t1386;
    let t17634 = 8.0_f64 / 45.0_f64 * t5077 * t5078 * t5232;
    (t17620, t17624, t17627, t17631, t17634)
}
