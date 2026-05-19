//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 941/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk941<F: Float>(t41231: F, t41244: F, t39118: F, t959: F, t39123: F, t13847: F, t2684: F, t7354: F, t41295: F, t41299: F, t41312: F, t41316: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47377 = F::cast_from(0.63904876589867916128e-1_f64) * t41231;
    let t47378 = F::cast_from(0.63904876589867916128e-1_f64) * t41244;
    let t47379 = t39118 * t959;
    let t47381 = t39123 * t959;
    let t47389 = t2684 * t7354 * t13847;
    let t47402 = F::cast_from(0.63904876589867916128e-1_f64) * t41295;
    let t47403 = F::cast_from(0.63904876589867916128e-1_f64) * t41299;
    let t47405 = F::cast_from(0.63904876589867916128e-1_f64) * t41312;
    let t47406 = F::cast_from(0.63904876589867916128e-1_f64) * t41316;
    (t47377, t47378, t47379, t47381, t47389, t47402, t47403, t47405, t47406)
}
