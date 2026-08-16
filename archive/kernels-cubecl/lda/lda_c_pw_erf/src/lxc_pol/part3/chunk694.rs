//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 694/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk694<F: Float>(t100: F, t3222: F, t125: F, t143: F, t1735: F, t3251: F, t405: F, t4117: F, t4122: F, t4125: F, t4129: F, t4132: F, t4136: F, t4140: F, t4144: F, t4252: F, t4280: F) -> (F, F) {
    let t4283 = t3222 * t100;
    let t4286 = F::cast_from(9.0_f64) * t4117 * t1735 - F::cast_from(0.0008717022455366076_f64) * t4122 - F::cast_from(0.0017434044910732151_f64) * t4125 - t4129 + F::cast_from(0.004067943812504169_f64) * t4132 + t4136 - t4140 - t4144 + F::cast_from(3.0_f64) * t405 * t143 * t3251 + (t4252 + t4280) * t125 + F::cast_from(6.0_f64) * t4283 * t143;
    (t4283, t4286)
}
