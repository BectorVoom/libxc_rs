//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1132/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1132<F: Float>(t41295: F, t41299: F, t43864: F, t43870: F, t43875: F, t43879: F, t43882: F, t43883: F, t43884: F, t43885: F, t43886: F, t43887: F) -> F {
    let t47402 = F::cast_from(0.63904876589867916128e-1_f64) * t41295;
    let t47403 = F::cast_from(0.63904876589867916128e-1_f64) * t41299;
    let t47404 = t43864 - F::cast_from(0.69017266717057349418e1_f64) * t43870 + t43875 - t43879 + t43882 + t43883 + t43884 - t43885 - t43886 + t43887 - t47402 - t47403;
    t47404
}
