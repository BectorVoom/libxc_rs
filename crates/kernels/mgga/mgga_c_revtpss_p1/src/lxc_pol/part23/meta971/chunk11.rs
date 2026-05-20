//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3290/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3290<F: Float>(t14141: F, t23037: F, t686: F, t72: F, t10049: F, t22863: F, t22954: F, t4118: F, t47967: F, t47971: F, t47979: F, t47981: F, t47985: F, t74935: F, t74943: F, t74945: F, t820: F) -> F {
    let t86401 = t14141 * t23037 * t72 * t686;
    let t86405 = -F::cast_from(0.58911598146606471821e-3_f64) * t47967 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4118 * t22954 + F::cast_from(0.91069445034239308177e-1_f64) * t47971 - t47979 - t47981 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t10049 * t22863 - F::cast_from(0.65854491829355115984e-1_f64) * t74935 + F::cast_from(0.43902994552903410658e-1_f64) * t47985 + F::cast_from(0.58544643236296698112e-1_f64) * t86401 + F::cast_from(0.32927245914677557992e-1_f64) * t74943 + F::cast_from(0.39029762157531132074e-2_f64) * t74945;
    t86405
}
