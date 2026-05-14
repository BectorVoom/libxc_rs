//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1166/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1166<F: Float>(t11531: F, t14015: F, t8991: F, t9035: F, t11754: F, t4039: F, t56998: F, t57000: F, t57002: F, t57004: F, t57006: F, t57009: F, t57011: F, t57013: F, t57015: F, t57017: F) -> (F,) {
    let t57019 = t14015 * t11531;
    let t57021 = t9035 * t8991;
    let t57023 = t4039 * t11754;
    let t57025 = -t56998 / 24.0 + 7.0 / 144.0 * t57000 - t57002 / 48.0 + 5.0 / 192.0 * t57004 + t57006 / 384.0 - t57009 / 96.0 - t57011 / 32.0 - t57013 / 48.0 - 5.0 / 96.0 * t57015 + t57017 / 768.0 + t57019 / 96.0 + t57021 / 24.0 + t57023 / 768.0;
    (t57025,)
}
