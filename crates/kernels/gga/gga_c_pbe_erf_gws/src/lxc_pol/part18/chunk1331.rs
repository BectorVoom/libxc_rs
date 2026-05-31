//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1331/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1331<F: Float>(t11754: F, t4039: F, t56998: F, t57000: F, t57002: F, t57004: F, t57006: F, t57009: F, t57011: F, t57013: F, t57015: F, t57017: F, t57019: F, t57021: F) -> F {
    let t57023 = t4039 * t11754;
    let t57025 = -t56998 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57000 - t57002 / F::cast_from(48.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t57004 + t57006 / F::cast_from(384.0_f64) - t57009 / F::cast_from(96.0_f64) - t57011 / F::cast_from(32.0_f64) - t57013 / F::cast_from(48.0_f64) - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t57015 + t57017 / F::cast_from(768.0_f64) + t57019 / F::cast_from(96.0_f64) + t57021 / F::cast_from(24.0_f64) + t57023 / F::cast_from(768.0_f64);
    t57025
}
