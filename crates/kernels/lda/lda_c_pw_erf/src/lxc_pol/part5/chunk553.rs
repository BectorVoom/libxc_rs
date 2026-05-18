//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 553/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk553<F: Float>(t169: F, t289: F, t2929: F, t274: F, t343: F, t39: F, t678: F, t1012: F, t385: F) -> (F, F, F, F) {
    let t2932 = F::new(0.031835665774679375) * t169 * t289 * t2929;
    let t2934 = F::new(1.279801625812305) * t343 * t274;
    let t2935 = t39 * t678;
    let t2940 = t1012 * t385;
    (t2932, t2934, t2935, t2940)
}
