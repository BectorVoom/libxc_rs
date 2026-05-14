//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 808/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk808<F: Float>(t103: F, t2060: F, t3082: F, t3396: F, t3413: F, t3414: F, t4635: F, t4639: F, t4642: F, t5002: F, t5003: F, t5006: F, t5032: F, t5034: F, t6147: F, t6152: F, t6156: F, t6162: F, t6167: F) -> (F,) {
    let t6175 = -0.015996296296296297 * t3082 + 0.013333333333333334 * t103 * t6147 - 0.002962962962962963 * t103 * t6152 - 0.008888888888888889 * t2060 * t6156 + 0.07198333333333333 * t6162 - 0.023994444444444443 * t6167 - 0.047988888888888886 * t4639 + t5002 - 0.014814814814814815 * t5003 + 0.017777777777777778 * t5006 - 0.03199259259259259 * t4635 + 0.047988888888888886 * t4642 - t3413 - t3414 - t5032 + t5034 - 0.007407407407407408 * t3396;
    (t6175,)
}
