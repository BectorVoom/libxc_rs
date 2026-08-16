//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1155/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1155<F: Float>(t31168: F, t3411: F, t247: F, t251: F, t25395: F, t256: F, t48313: F, t48315: F, t48316: F, t48318: F, t48320: F, t48321: F, t48330: F, t48359: F, t48363: F, t48367: F) -> (F, F) {
    let t48369 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t31168 * t3411;
    let t48370 = -t48313 + t48315 + t48316 - F::cast_from(32.0_f64) / F::cast_from(405.0_f64) * t25395 + t48318 + t48320 + t48321 * t247 * t251 * t256 / F::cast_from(3.0_f64) + t48330 + t48359 - t48363 + t48367 + t48369;
    (t48369, t48370)
}
