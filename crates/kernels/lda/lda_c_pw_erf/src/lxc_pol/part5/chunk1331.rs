//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1331/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1331<F: Float>(t21719: F, t21721: F, t21725: F, t21726: F, t21727: F, t21728: F, t21729: F, t21730: F, t21731: F, t21732: F, t21733: F, t21734: F, t21738: F) -> F {
    let t23269 = -t21719 - t21721 + t21725 - t21726 + t21727 - t21728 - t21729 + t21730 - t21731 + t21732 + t21733 + t21734 + t21738;
    t23269
}
