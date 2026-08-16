//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1132/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1132(t41295: f64, t41299: f64, t43864: f64, t43870: f64, t43875: f64, t43879: f64, t43882: f64, t43883: f64, t43884: f64, t43885: f64, t43886: f64, t43887: f64) -> f64 {
    let t47402 = 0.63904876589867916128e-1_f64 * t41295;
    let t47403 = 0.63904876589867916128e-1_f64 * t41299;
    let t47404 = t43864 - 0.69017266717057349418e1_f64 * t43870 + t43875 - t43879 + t43882 + t43883 + t43884 - t43885 - t43886 + t43887 - t47402 - t47403;
    t47404
}
