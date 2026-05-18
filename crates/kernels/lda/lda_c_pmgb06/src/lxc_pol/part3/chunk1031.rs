//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1031/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1031<F: Float>(t12252: F, t132: F, t137: F, t1594: F, t12227: F, t12230: F, t12233: F, t12235: F, t12237: F, t12240: F, t12242: F, t12244: F, t12246: F, t12249: F, t12251: F) -> (F, F) {
    let t12256 = t132 * t137 * t12252 * t1594 / F::new(5.0);
    let t12257 = -F::new(2.0) / F::new(9.0) * t12227 + t12230 + t12233 + t12235 - t12237 + t12240 + t12242 - t12244 + t12246 - t12249 + t12251 + t12256;
    (t12256, t12257)
}
