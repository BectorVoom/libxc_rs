//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1048/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1048<F: Float>(t1472: F, t4810: F, t4813: F, t1948: F, t2973: F, t1319: F, t571: F, t12252: F, t12257: F, t12259: F, t12261: F, t12263: F, t12267: F, t12271: F, t12273: F, t12275: F, t12277: F) -> (F, F, F, F, F) {
    let t12279 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1472 * t4810;
    let t12281 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t1472 * t4813;
    let t12282 = t1948 * t2973;
    let t12285 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t571 * t1319 * t12282;
    let t12286 = t12252 - t12257 + t12259 + t12261 + t12263 + t12267 + t12271 + t12273 - t12275 - t12277 - t12279 + t12281 - t12285;
    (t12279, t12281, t12282, t12285, t12286)
}
