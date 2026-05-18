//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1199/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1199<F: Float>(t12246: F, t12249: F, t12251: F, t12256: F, t12260: F, t12265: F, t12267: F, t12269: F, t12271: F, t12273: F, t12275: F, t10687: F, t10690: F, t12277: F, t12279: F, t12282: F, t12300: F, t12302: F, t12315: F, t12415: F, t12417: F, t12435: F, t12437: F) -> (F, F) {
    let t14366 = t12246 - t12249 + t12251 + t12256 - t12260 - t12265 - t12267 - t12269 - t12271 - t12273 + t12275;
    let t14367 = t12277 + t12279 + t12282 + t12300 + t12302 - t10687 + t10690 + t12315 + t12415 + t12417 + t12435 + t12437;
    (t14366, t14367)
}
