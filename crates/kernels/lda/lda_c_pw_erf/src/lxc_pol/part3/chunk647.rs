//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 647/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk647<F: Float>(t125: F, t143: F, t1735: F, t3251: F, t405: F, t4117: F, t4122: F, t4125: F, t4129: F, t4132: F, t4136: F, t4140: F, t4144: F, t4252: F, t4280: F, t4283: F) -> (F,) {
    let t4286 = 9.0 * t4117 * t1735 - 0.0008717022455366076 * t4122 - 0.0017434044910732151 * t4125 - t4129 + 0.004067943812504169 * t4132 + t4136 - t4140 - t4144 + 3.0 * t405 * t143 * t3251 + (t4252 + t4280) * t125 + 6.0 * t4283 * t143;
    (t4286,)
}
