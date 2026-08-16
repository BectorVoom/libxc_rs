//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 391/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk391<F: Float>(t3221: F, t3220: F, t3091: F, t713: F, t928: F, t3217: F, t3218: F, t871: F, t931: F, t295: F, t3113: F, t471: F) -> (F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t3222 = t3221 * pi;
    let t3223 = t3220 * t3222;
    let t3225 = t713 * t3091;
    let t3226 = t3225 * t928;
    let t3227 = t3226 / F::cast_from(256.0_f64);
    let t3228 = t3217 - F::cast_from(9.0_f64) / F::cast_from(8192.0_f64) * t3218 + F::cast_from(3.0_f64) / F::cast_from(8192.0_f64) * t3223 - t3227;
    let t3230 = t931 * t871;
    let t3232 = t295 * t3113;
    let t3234 = t3228 * t471 + t3230 / F::cast_from(2.0_f64) + t3217 - t3227 - t3232 / F::cast_from(2.0_f64);
    (t3222, t3223, t3225, t3226, t3228, t3230, t3232, t3234)
}
