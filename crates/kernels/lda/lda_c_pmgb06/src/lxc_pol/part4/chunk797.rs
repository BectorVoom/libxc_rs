//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 797/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk797<F: Float>(t2871: F, t5336: F, t493: F, t5301: F, t5304: F, t5307: F, t5309: F, t5311: F, t5315: F, t5317: F, t5321: F, t5324: F, t5325: F, t5328: F, t5330: F, t5332: F, t5335: F) -> (F, F, F) {
    let t5337 = t2871 * t5336;
    let t5339 = F::new(2.0) / F::new(45.0) * t493 * t5337;
    let t5340 = t5301 - t5304 + t5307 + t5309 + t5311 + t5315 + t5317 + t5321 - t5324 - t5325 - t5328 - t5330 + t5332 + t5335 + t5339;
    (t5337, t5339, t5340)
}
