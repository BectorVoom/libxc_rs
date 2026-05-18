//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 239/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk239<F: Float>(t108: F, t348: F, t352: F, t659: F, t661: F, t266: F, t9: F) -> (F, F) {
    let t665 = (F::new(4.0) / F::new(3.0) * t659 * t348 + F::new(4.0) / F::new(3.0) * t661 * t352) * t108;
    let t668 = t266 * t9;
    (t665, t668)
}
