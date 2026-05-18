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
    let t57025 = -t56998 / F::new(24.0) + F::new(7.0) / F::new(144.0) * t57000 - t57002 / F::new(48.0) + F::new(5.0) / F::new(192.0) * t57004 + t57006 / F::new(384.0) - t57009 / F::new(96.0) - t57011 / F::new(32.0) - t57013 / F::new(48.0) - F::new(5.0) / F::new(96.0) * t57015 + t57017 / F::new(768.0) + t57019 / F::new(96.0) + t57021 / F::new(24.0) + t57023 / F::new(768.0);
    t57025
}
