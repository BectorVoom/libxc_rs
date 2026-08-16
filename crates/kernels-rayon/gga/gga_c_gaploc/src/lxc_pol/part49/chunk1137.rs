//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1137/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1137(t43924: f64, t43926: f64, t43928: f64, t43931: f64, t43935: f64, t43938: f64, t43941: f64, t43944: f64, t47442: f64, t47445: f64, t47448: f64, t47450: f64) -> f64 {
    let t47452 = t43924 - t43926 - t43928 - t43931 - t43935 - t43938 - 0.25025342966295298669e1_f64 * t43941 - 0.92023022289409799224e1_f64 * t43944 - t47442 - 0.35750489951850426669e0_f64 * t47445 + 0.69017266717057349418e1_f64 * t47448 - 0.29792074959875355558e-1_f64 * t47450;
    t47452
}
