//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 749/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk749<F: Float>(t5317: F, t5321: F, t5324: F, t5325: F, t5328: F, t5330: F, t5332: F, t5335: F, t5339: F, t5342: F, t5347: F, t5349: F, t5352: F, t5354: F, t5356: F, t187: F, t2342: F) -> (F, F) {
    let t5672 = t5317 + t5321 - t5324 - t5325 - t5328 - t5330 + t5332 + t5335 + t5339 - t5342 + t5347 - t5349 + t5352 + t5354 - t5356;
    let t5674 = 8.0 / 3.0 * t2342 * t187;
    (t5672, t5674)
}
