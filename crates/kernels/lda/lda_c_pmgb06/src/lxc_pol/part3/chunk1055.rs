//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1055/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1055<F: Float>(t13264: F, t13266: F, t13268: F, t13270: F, t13272: F, t13277: F, t13279: F, t13283: F, t13284: F, t13285: F, t13286: F, t10134: F, t13287: F, t13288: F, t13289: F, t13290: F, t13292: F, t13295: F, t13299: F, t13303: F, t13307: F, t13311: F, t13313: F) -> (F, F) {
    let t14420 = t13264 + t13266 + t13268 + t13270 + t13272 - t13277 - t13279 - t13283 - t13284 - t13285 + t13286;
    let t14421 = t13287 - t13288 + t13289 + t13290 - t13292 - t13295 + t13299 - t10134 + t13303 - t13307 - t13311 + t13313;
    (t14420, t14421)
}
