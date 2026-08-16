//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 690/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk690<F: Float>(t4176: F, t4177: F, t4179: F, t4180: F, t4210: F, t4211: F, t4213: F, t4236: F, t163: F, t1645: F, t169: F, t299: F) -> (F, F) {
    let t4239 = t4176 + t4177 + t4179 + t4180 + t4210 + t4211 + t4213 + t4236;
    let t4246 = t169 * t299 * t1645 * t163;
    (t4239, t4246)
}
