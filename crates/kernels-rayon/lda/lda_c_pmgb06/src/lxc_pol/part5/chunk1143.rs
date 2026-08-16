//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1143/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1143(t132: f64, t137: f64, t153: f64, t20715: f64, t20734: f64, t432: f64, t7813: f64, t10178: f64, t7811: f64, t17506: f64, t6613: f64, t802: f64) -> (f64, f64, f64, f64, f64) {
    let t20739 = t132 * t137 * (t20715 + t20734) * t153 / 30.0_f64;
    let t20741 = t432 * t7813 / 5.0_f64;
    let t20745 = t132 * t137 * t10178 * t7811 / 5.0_f64;
    let t20746 = 2.0_f64 / 15.0_f64 * t17506;
    let t20747 = t802 * t6613;
    (t20739, t20741, t20745, t20746, t20747)
}
