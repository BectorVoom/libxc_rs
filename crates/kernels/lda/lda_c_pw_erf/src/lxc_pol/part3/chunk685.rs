//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 685/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk685<F: Float>(t185: F, t4729: F, t1931: F, t230: F, t610: F, t838: F, t2007: F, t3794: F, t2119: F, t518: F) -> (F, F, F, F, F) {
    let t4730 = t185 * t4729;
    let t4731 = 4.0 / 135.0 * t4730;
    let t4733 = 8.0 / 3.0 * t1931 * t230;
    let t4734 = t838 * t610;
    let t4737 = 16.0 / 45.0 * t3794 * t2007;
    let t4738 = t2119 * t518;
    (t4731, t4733, t4734, t4737, t4738)
}
