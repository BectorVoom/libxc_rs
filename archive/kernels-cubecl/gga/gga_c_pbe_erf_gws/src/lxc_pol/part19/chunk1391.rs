//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1391/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1391<F: Float>(t54352: F, t54356: F, t54381: F, t55608: F, t55620: F, t57195: F, t57197: F, t57199: F, t57201: F, t57204: F, t57206: F, t57208: F, t57210: F) -> F {
    let t58765 = -F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t54352 + t55608 - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t54356 + t55620 - t57195 / F::cast_from(192.0_f64) - t57197 / F::cast_from(96.0_f64) - t57199 / F::cast_from(96.0_f64) - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t54381 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57201 + t57204 / F::cast_from(12.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57206 + t57208 / F::cast_from(12.0_f64) + t57210 / F::cast_from(8.0_f64);
    t58765
}
