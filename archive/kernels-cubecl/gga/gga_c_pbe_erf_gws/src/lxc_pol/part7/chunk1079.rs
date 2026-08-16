//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1079/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1079<F: Float>(t133: F, t19298: F, t19301: F, t19216: F, t19219: F, t19229: F, t19236: F, t19240: F, t19249: F, t19264: F, t19282: F, t19286: F, t19290: F, t19294: F, t19312: F, t19420: F) -> F {
    let t19422 = t133 * t19298;
    let t19424 = t133 * t19301;
    let t19426 = -t19216 + t19219 + t19229 - t19236 - t19240 - t19249 - t19264 + t19282 + t19286 + t19290 + t19294 + t19312 + F::cast_from(0.7152465185185185185e1_f64) * t19420 - F::cast_from(0.45980133333333333333e1_f64) * t19422 + F::cast_from(0.22990066666666666667e1_f64) * t19424;
    t19426
}
