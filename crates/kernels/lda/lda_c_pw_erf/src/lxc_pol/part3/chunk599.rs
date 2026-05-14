//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 599/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk599<F: Float>(t3783: F, t524: F, t519: F, t188: F, t9: F) -> (F, F, F, F) {
    let t3784 = t3783 * t524;
    let t3785 = t519 * t3784;
    let t3786 = 8.0 / 135.0 * t3785;
    let t3787 = t9 * t188;
    (t3784, t3785, t3786, t3787)
}
