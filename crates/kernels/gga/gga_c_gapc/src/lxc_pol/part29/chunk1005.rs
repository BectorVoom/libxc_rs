//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1005/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1005<F: Float>(t11923: F, t30158: F, t3402: F, t10036: F, t11872: F, t11960: F, t869: F, t9555: F, t11965: F, t9741: F, t34205: F, t34207: F, t34209: F, t34211: F, t34214: F, t34217: F, t34219: F) -> (F,) {
    let t34222 = t3402 * t11923 * t30158;
    let t34224 = t11872 * t10036;
    let t34227 = t869 * t11960 * t9555;
    let t34230 = t869 * t11965 * t9741;
    let t34232 = 0.20240885416666666668e-4 * t34205 + 0.10120442708333333334e-3 * t34207 + 0.16217772716043213195e-2 * t34209 + 0.12290803273518880209e-7 * t34211 + 0.10860115658064651693e-4 * t34214 - 0.11049275749843950005e-7 * t34217 - 0.78582449132890172433e-8 * t34219 - 0.4834058140556728127e-8 * t34222 - 0.67528199161846004232e-6 * t34224 + 0.42168511284722222224e-6 * t34227 - 0.36897447374131944446e-6 * t34230;
    (t34232,)
}
