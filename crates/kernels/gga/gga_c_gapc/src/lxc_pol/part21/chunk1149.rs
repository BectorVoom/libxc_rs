//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1149/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1149<F: Float>(t11965: F, t869: F, t9741: F, t34205: F, t34207: F, t34209: F, t34211: F, t34214: F, t34217: F, t34219: F, t34222: F, t34224: F, t34227: F) -> F {
    let t34230 = t869 * t11965 * t9741;
    let t34232 = F::cast_from(0.20240885416666666668e-4_f64) * t34205 + F::cast_from(0.10120442708333333334e-3_f64) * t34207 + F::cast_from(0.16217772716043213195e-2_f64) * t34209 + F::cast_from(0.12290803273518880209e-7_f64) * t34211 + F::cast_from(0.10860115658064651693e-4_f64) * t34214 - F::cast_from(0.11049275749843950005e-7_f64) * t34217 - F::cast_from(0.78582449132890172433e-8_f64) * t34219 - F::cast_from(0.4834058140556728127e-8_f64) * t34222 - F::cast_from(0.67528199161846004232e-6_f64) * t34224 + F::cast_from(0.42168511284722222224e-6_f64) * t34227 - F::cast_from(0.36897447374131944446e-6_f64) * t34230;
    t34232
}
