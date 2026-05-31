//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1349/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1349<F: Float>(t12080: F, t14101: F, t54355: F, t54378: F, t55607: F, t55609: F, t55623: F, t57195: F, t57197: F, t57199: F, t57201: F, t57204: F, t57206: F, t57208: F) -> F {
    let t57210 = t14101 * t12080;
    let t57212 = -t55607 + t54355 - t55609 + t54378 - t57195 / F::cast_from(384.0_f64) - t57197 / F::cast_from(192.0_f64) - t57199 / F::cast_from(192.0_f64) - t55623 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57201 + t57204 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57206 + t57208 / F::cast_from(24.0_f64) + t57210 / F::cast_from(16.0_f64);
    t57212
}
