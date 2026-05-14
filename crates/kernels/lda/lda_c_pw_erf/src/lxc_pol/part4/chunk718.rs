//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 718/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk718<F: Float>(t1329: F, t4738: F, t231: F, t4705: F, t4707: F, t4708: F, t4709: F, t4710: F, t4714: F, t4718: F, t4719: F, t4721: F, t4726: F, t4728: F, t4731: F, t4733: F, t4734: F, t4737: F) -> (F, F) {
    let t4740 = 16.0 / 45.0 * t4738 * t1329;
    let t4741 = t4705 + t4707 + t4708 + t4709 - t4710 + 4.0 / 3.0 * t4714 * t231 + t4718 + 4.0 / 3.0 * t4719 + t4721 + t4726 - t4728 + t4731 + t4733 + 8.0 / 3.0 * t4734 + t4737 + t4740;
    (t4740, t4741)
}
