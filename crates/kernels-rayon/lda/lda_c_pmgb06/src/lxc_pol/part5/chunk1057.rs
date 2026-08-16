//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1057/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1057(t1972: f64, t6748: f64, t6752: f64, t16029: f64, t16031: f64, t16033: f64, t19642: f64, t19644: f64, t19658: f64, t19660: f64, t19662: f64, t19664: f64, t19666: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19668 = 4.0_f64 / 15.0_f64 * t1972 * t6748;
    let t19670 = 2.0_f64 / 9.0_f64 * t1972 * t6752;
    let t19671 = 2.0_f64 / 45.0_f64 * t16029;
    let t19672 = 4.0_f64 / 45.0_f64 * t16031;
    let t19673 = 2.0_f64 / 27.0_f64 * t16033;
    let t19674 = t19642 + t19644 + t19658 + t19660 + t19662 + t19664 - t19666 - t19668 + t19670 - t19671 - t19672 + t19673;
    (t19668, t19670, t19671, t19672, t19673, t19674)
}
