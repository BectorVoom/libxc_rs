//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 767/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk767<F: Float>(t188: F, t5632: F, t183: F, t4463: F, t1798: F, t539: F, t4798: F, t4800: F, t4805: F, t4807: F, t4809: F, t4812: F, t4814: F, t4819: F, t4821: F, t4823: F, t4825: F, t4827: F) -> (F, F, F, F, F) {
    let t5633 = t5632 * t188;
    let t5635 = t4463 * t183;
    let t5638 = t1798 * t539;
    let t5640 = 8.0 / 3.0 * t5638 * t188;
    let t5641 = -t4798 - t4800 - t4805 + t4807 + t4809 + t4812 + t4814 + 4.0 / 3.0 * t5633 + 4.0 / 3.0 * t5635 * t188 + t5640 - t4819 - t4821 - t4823 - t4825 - t4827;
    (t5633, t5635, t5638, t5640, t5641)
}
