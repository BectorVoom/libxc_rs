//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 727/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk727(t409: f64, t441: f64, t154: f64, t443: f64, t132: f64, t4792: f64, t4794: f64, t4796: f64, t4798: f64, t4800: f64, t4805: f64, t4807: f64, t4809: f64, t4812: f64, t4814: f64, t4819: f64, t4821: f64, t4823: f64, t4825: f64, t4827: f64) -> (f64, f64, f64, f64, f64) {
    let t4828 = t409 * t441;
    let t4829 = t154 * t443;
    let t4830 = t4828 * t4829;
    let t4832 = 2.0_f64 / 45.0_f64 * t132 * t4830;
    let t4833 = t4792 + t4794 + t4796 - t4798 - t4800 - t4805 + t4807 + t4809 + t4812 + t4814 - t4819 - t4821 - t4823 - t4825 - t4827 + t4832;
    (t4828, t4829, t4830, t4832, t4833)
}
