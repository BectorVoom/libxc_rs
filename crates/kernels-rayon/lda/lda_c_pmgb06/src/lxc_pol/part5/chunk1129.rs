//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1129/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1129(t1826: f64, t2624: f64, t5068: f64, t1821: f64, t5138: f64, t17598: f64, t1911: f64, t2653: f64, t20533: f64, t20536: f64, t20539: f64, t20541: f64, t20543: f64, t20545: f64, t20548: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20551 = 2.0_f64 / 15.0_f64 * t5068 * t2624 * t1826;
    let t20554 = t5138 * t2624 * t1821 / 9.0_f64;
    let t20557 = 4.0_f64 / 15.0_f64 * t5068 * t17598 * t1911;
    let t20560 = 4.0_f64 / 15.0_f64 * t5068 * t2653 * t1826;
    let t20563 = 2.0_f64 / 9.0_f64 * t5138 * t2653 * t1821;
    let t20564 = t20533 + t20536 - t20539 + t20541 + t20543 - t20545 + t20548 + t20551 - t20554 + t20557 + t20560 - t20563;
    (t20551, t20554, t20557, t20560, t20563, t20564)
}
