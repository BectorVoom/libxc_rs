//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 615/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk615<F: Float>(t2896: F, t3002: F, t3063: F, t3160: F, t3207: F, t3268: F, t3318: F, t3464: F, t117: F, t123: F, t550: F, t740: F) -> (F, F) {
    let t3467 = t2896 + t3002 + t3063 + t3160 + t3207 + t3268 + t3318 + t3464;
    let t3474 = t123 * t740 * t550 * t117;
    (t3467, t3474)
}
