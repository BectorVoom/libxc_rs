//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 968/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk968<F: Float>(t168: F, t2782: F, t671: F, t153: F, t3196: F, t474: F, t2869: F, t678: F, t1210: F, t1534: F, t4107: F, t632: F) -> (F, F, F, F, F) {
    let t11198 = t168 * t2782 * t671;
    let t11201 = t153 * t474 * t3196;
    let t11204 = t153 * t2869 * t678;
    let t11211 = t168 * t1210 * t1534;
    let t11215 = t4107 * t632;
    (t11198, t11201, t11204, t11211, t11215)
}
