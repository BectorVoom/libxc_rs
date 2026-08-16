//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 792/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk792<F: Float>(t3881: F, t3884: F, t3888: F, t3942: F, t3944: F, t3946: F, t3949: F, t3954: F, t3956: F, t3959: F, t3962: F, t3965: F, t3968: F) -> F {
    let t7423 = t3881 - t3884 - t3888 + t3942 - t3944 - t3946 - t3949 - t3954 + t3956 - t3959 - t3962 + t3965 + t3968;
    t7423
}
