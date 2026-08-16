//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1144/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1144(t20747: f64, t27: f64, t545: f64, t7704: f64, t17544: f64, t20684: f64, t20689: f64, t20692: f64, t20694: f64, t20739: f64, t20741: f64, t20745: f64, t20746: f64) -> (f64, f64) {
    let t20748 = 2.0_f64 / 15.0_f64 * t20747;
    let t20750 = t7704 * t27 * t545;
    let t20753 = t20684 + t20689 + t20692 + t20694 + t20739 - t20741 - t20745 - t20746 - t20748 + 0.10821041362364843_f64 * t20750 + 0.3246312408709453_f64 * t17544;
    (t20748, t20753)
}
