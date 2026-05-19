//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1137/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1137<F: Float>(t43924: F, t43926: F, t43928: F, t43931: F, t43935: F, t43938: F, t43941: F, t43944: F, t47442: F, t47445: F, t47448: F, t47450: F) -> F {
    let t47452 = t43924 - t43926 - t43928 - t43931 - t43935 - t43938 - F::cast_from(0.25025342966295298669e1_f64) * t43941 - F::cast_from(0.92023022289409799224e1_f64) * t43944 - t47442 - F::cast_from(0.35750489951850426669e0_f64) * t47445 + F::cast_from(0.69017266717057349418e1_f64) * t47448 - F::cast_from(0.29792074959875355558e-1_f64) * t47450;
    t47452
}
