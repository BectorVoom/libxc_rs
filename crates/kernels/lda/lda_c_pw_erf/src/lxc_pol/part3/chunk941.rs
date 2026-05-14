//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 941/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk941<F: Float>(t10015: F, t5148: F, t739: F, t944: F, t348: F, t3965: F, t5147: F, t5136: F, t945: F, t12031: F, t12389: F, t3619: F, t4506: F, t5151: F, t10011: F, t5138: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12444 = 16.0 / 9.0 * t10015 * t5148;
    let t12445 = t739 * t944;
    let t12446 = t12445 * t348;
    let t12449 = 8.0 / 9.0 * t3965 * t5147 * t12446;
    let t12450 = t5136 * t945;
    let t12453 = 8.0 / 9.0 * t3965 * t5147 * t12450;
    let t12456 = 64.0 / 27.0 * t3965 * t12031 * t12389;
    let t12459 = 8.0 / 15.0 * t4506 * t5151 * t3619;
    let t12460 = t10011 * t5138;
    (t12444, t12445, t12446, t12449, t12450, t12453, t12456, t12459, t12460)
}
