//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1258/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1258<F: Float>(t54329: F, t14058: F, t3279: F, t1158: F, t51395: F, t3268: F, t1140: F, t14083: F, t3190: F, t3206: F, t2407: F, t26623: F, t858: F) -> (F, F, F, F, F, F, F) {
    let t54330 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t54329;
    let t54344 = t14058 * t3279;
    let t54345 = F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t54344;
    let t54352 = t51395 * t1158;
    let t54354 = t14058 * t3268;
    let t54355 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54354;
    let t54356 = t14083 * t1140;
    let t54359 = t3206 * t3190;
    let t54373 = t2407 * t858 * t26623;
    (t54330, t54345, t54352, t54355, t54356, t54359, t54373)
}
