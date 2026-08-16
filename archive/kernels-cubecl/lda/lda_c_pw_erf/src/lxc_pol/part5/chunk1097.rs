//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1097/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1097<F: Float>(t127: F, t14797: F, t14803: F, t14814: F, t14817: F, t20396: F, t20397: F, t20403: F, t20406: F, t20409: F, t20412: F, t411: F, t7918: F, t9037: F) -> F {
    let t20417 = F::cast_from(5.87616_f64) * t14797 + t14803 + t14814 + t14817 + t20396 + F::cast_from(5.87616_f64) * t127 * t20397 * t411 + t20403 - t20406 + t20409 + t20412 + F::cast_from(176.2848_f64) * t127 * t9037 * t7918 * t411;
    t20417
}
