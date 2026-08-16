//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1033/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1033(t2778: f64, t301: f64, t413: f64, t398: f64, t642: f64, t113: f64, t1126: f64, t1147: f64, t123: f64, t317: f64, t4001: f64, t701: f64) -> (f64, f64, f64, f64, f64) {
    let t10643 = 0.026861343269868797_f64 * t2778 * t413 * t301;
    let t10644 = t642 * t398;
    let t10646 = t10644 * t113 * t301;
    let t10657 = t123 * t1147 * t1126 * t317;
    let t10661 = t123 * t4001 * t701 * t317;
    (t10643, t10644, t10646, t10657, t10661)
}
