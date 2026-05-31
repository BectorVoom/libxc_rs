//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1079/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1079<F: Float>(t42953: F, t47343: F, t47347: F, t47351: F, t47355: F, t47359: F, t47363: F, t47364: F, t47366: F, t47368: F, t47369: F, t16576: F, t39: F) -> (F, F) {
    let t47370 = t47343 + t47347 + t47351 - t47355 - t47359 - t47363 - t47364 + t47366 - F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t42953 - t47368 - t47369;
    let t47371 = -t39 - t16576;
    (t47370, t47371)
}
