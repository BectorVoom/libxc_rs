//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 950/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk950<F: Float>(t6267: F, t6270: F, t6272: F, t6274: F, t6277: F, t6279: F, t6281: F, t6284: F, t6289: F, t6291: F, t6294: F, t6296: F, t6299: F, t6302: F, t6305: F) -> F {
    let t7184 = t6267 - t6270 + t6272 + t6274 + t6277 + t6279 + t6281 + t6284 - t6289 - t6291 - t6294 - t6296 - t6299 + t6302 + t6305;
    t7184
}
