//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 805/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk805(t1409: f64, t794: f64, t188: f64, t183: f64, t4463: f64, t1798: f64, t539: f64, t4798: f64, t4800: f64, t4805: f64, t4807: f64, t4809: f64, t4812: f64, t4814: f64, t4819: f64, t4821: f64, t4823: f64, t4825: f64, t4827: f64) -> (f64, f64, f64, f64) {
    let t5632 = t794 * t1409;
    let t5633 = t5632 * t188;
    let t5635 = t4463 * t183;
    let t5638 = t1798 * t539;
    let t5640 = 8.0_f64 / 3.0_f64 * t5638 * t188;
    let t5641 = -t4798 - t4800 - t4805 + t4807 + t4809 + t4812 + t4814 + 4.0_f64 / 3.0_f64 * t5633 + 4.0_f64 / 3.0_f64 * t5635 * t188 + t5640 - t4819 - t4821 - t4823 - t4825 - t4827;
    (t5632, t5635, t5638, t5641)
}
