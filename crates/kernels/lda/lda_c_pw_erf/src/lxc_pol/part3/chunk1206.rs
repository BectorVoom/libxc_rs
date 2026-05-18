//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1206/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1206<F: Float>(t1313: F, t1995: F, t2961: F, t519: F, t1446: F, t5222: F, t1245: F, t2098: F, t3402: F, t940: F, t14188: F, t14191: F, t14194: F, t14197: F, t14199: F, t14203: F, t14208: F, t14210: F, t14212: F, t14216: F) -> (F, F, F, F) {
    let t14220 = F::new(4.0) / F::new(45.0) * t519 * t1313 * t1995 * t2961;
    let t14222 = F::new(4.0) / F::new(9.0) * t1446 * t5222;
    let t14227 = F::new(4.0) / F::new(9.0) * t519 * t3402 * t2098 * t1245 * t940;
    let t14228 = -t14188 - t14191 - t14194 + t14197 + t14199 + t14203 + t14208 + t14210 - t14212 - t14216 - t14220 - t14222 - t14227;
    (t14220, t14222, t14227, t14228)
}
