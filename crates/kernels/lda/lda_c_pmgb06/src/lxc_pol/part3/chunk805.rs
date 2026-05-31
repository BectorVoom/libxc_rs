//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 805/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk805<F: Float>(t1409: F, t794: F, t188: F, t183: F, t4463: F, t1798: F, t539: F, t4798: F, t4800: F, t4805: F, t4807: F, t4809: F, t4812: F, t4814: F, t4819: F, t4821: F, t4823: F, t4825: F, t4827: F) -> (F, F, F, F) {
    let t5632 = t794 * t1409;
    let t5633 = t5632 * t188;
    let t5635 = t4463 * t183;
    let t5638 = t1798 * t539;
    let t5640 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5638 * t188;
    let t5641 = -t4798 - t4800 - t4805 + t4807 + t4809 + t4812 + t4814 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5633 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5635 * t188 + t5640 - t4819 - t4821 - t4823 - t4825 - t4827;
    (t5632, t5635, t5638, t5641)
}
