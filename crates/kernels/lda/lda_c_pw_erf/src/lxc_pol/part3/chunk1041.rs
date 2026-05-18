//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1041/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1041<F: Float>(t1313: F, t3545: F, t519: F, t789: F, t10467: F, t2030: F, t1472: F, t4838: F, t1308: F, t3655: F, t571: F, t816: F) -> (F, F, F, F) {
    let t12194 = F::new(4.0) / F::new(45.0) * t519 * t1313 * t789 * t3545;
    let t12196 = t519 * t10467 * t2030;
    let t12197 = F::new(8.0) / F::new(135.0) * t12196;
    let t12199 = F::new(4.0) / F::new(15.0) * t1472 * t4838;
    let t12203 = F::new(4.0) / F::new(45.0) * t571 * t1308 * t816 * t3655;
    (t12194, t12197, t12199, t12203)
}
