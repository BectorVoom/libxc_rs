//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 743/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk743<F: Float>(t409: F, t441: F, t154: F, t443: F, t132: F, t4792: F, t4794: F, t4796: F, t4798: F, t4800: F, t4805: F, t4807: F, t4809: F, t4812: F, t4814: F, t4819: F, t4821: F, t4823: F, t4825: F, t4827: F) -> (F, F, F, F, F) {
    let t4828 = t409 * t441;
    let t4829 = t154 * t443;
    let t4830 = t4828 * t4829;
    let t4832 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t132 * t4830;
    let t4833 = t4792 + t4794 + t4796 - t4798 - t4800 - t4805 + t4807 + t4809 + t4812 + t4814 - t4819 - t4821 - t4823 - t4825 - t4827 + t4832;
    (t4828, t4829, t4830, t4832, t4833)
}
