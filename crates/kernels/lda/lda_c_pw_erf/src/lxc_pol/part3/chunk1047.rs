//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1047/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1047<F: Float>(t14188: F, t14191: F, t14194: F, t14197: F, t14199: F, t14203: F, t14208: F, t14210: F, t14212: F, t14216: F, t14220: F, t14222: F, t14227: F, t1446: F, t5247: F, t11687: F, t1991: F, t519: F) -> (F, F, F) {
    let t14228 = -t14188 - t14191 - t14194 + t14197 + t14199 + t14203 + t14208 + t14210 - t14212 - t14216 - t14220 - t14222 - t14227;
    let t14230 = 4.0 / 9.0 * t1446 * t5247;
    let t14233 = 4.0 / 27.0 * t519 * t1991 * t11687;
    (t14228, t14230, t14233)
}
