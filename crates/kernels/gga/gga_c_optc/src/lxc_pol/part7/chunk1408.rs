//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1408/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1408<F: Float>(t1210: F, t9227: F, t1214: F, t9340: F, t3268: F, t26181: F, t26184: F, t26188: F, t26192: F, t26200: F, t26203: F, t26206: F, t26209: F, t26212: F, t277: F, t2911: F, t95: F) -> F {
    let t28040 = t1210 * t9227;
    let t28042 = t9340 * t1214;
    let t28044 = t3268 * t3268;
    let t28049 = -t26181 - t26184 - t26188 + t26192 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t28040 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28042 + t26200 - t26203 - t26206 + t26209 + t26212 - F::cast_from(0.77534644304710291488e-2_f64) * t95 * t277 * t28044 * t2911;
    t28049
}
