//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1152/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1152(t11965: f64, t869: f64, t9741: f64, t34205: f64, t34207: f64, t34209: f64, t34211: f64, t34214: f64, t34217: f64, t34219: f64, t34222: f64, t34224: f64, t34227: f64) -> f64 {
    let t34230 = t869 * t11965 * t9741;
    let t34232 = 0.20240885416666666668e-4_f64 * t34205 + 0.10120442708333333334e-3_f64 * t34207 + 0.16217772716043213195e-2_f64 * t34209 + 0.12290803273518880209e-7_f64 * t34211 + 0.10860115658064651693e-4_f64 * t34214 - 0.11049275749843950005e-7_f64 * t34217 - 0.78582449132890172433e-8_f64 * t34219 - 0.4834058140556728127e-8_f64 * t34222 - 0.67528199161846004232e-6_f64 * t34224 + 0.42168511284722222224e-6_f64 * t34227 - 0.36897447374131944446e-6_f64 * t34230;
    t34232
}
