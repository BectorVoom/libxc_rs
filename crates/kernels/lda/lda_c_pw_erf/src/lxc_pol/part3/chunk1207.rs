//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1207/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1207<F: Float>(t1446: F, t5247: F, t11687: F, t1991: F, t519: F, t3854: F, t4693: F, t571: F, t4671: F, t4794: F, t10527: F, t219: F) -> (F, F, F, F, F) {
    let t14230 = F::new(4.0) / F::new(9.0) * t1446 * t5247;
    let t14233 = F::new(4.0) / F::new(27.0) * t519 * t1991 * t11687;
    let t14235 = t571 * t3854 * t4693;
    let t14236 = F::new(16.0) / F::new(45.0) * t14235;
    let t14238 = t571 * t4794 * t4671;
    let t14239 = F::new(16.0) / F::new(9.0) * t14238;
    let t14240 = t10527 * t219;
    (t14230, t14233, t14236, t14239, t14240)
}
