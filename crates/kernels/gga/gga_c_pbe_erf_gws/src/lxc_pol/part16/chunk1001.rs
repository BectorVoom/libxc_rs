//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1001/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1001<F: Float>(t4113: F, t840: F, t13988: F, t2409: F, t4097: F, t8734: F, t4099: F, t9270: F, t4088: F, t6781: F, t13973: F, t13977: F, t13985: F, t14002: F, t14114: F, t14119: F, t14123: F, t14128: F, t14130: F, t14133: F, t14139: F, t14141: F, t2408: F, t3066: F) -> (F, F, F, F, F, F) {
    let t14333 = t840 * t4113;
    let t14338 = 35.0 / 216.0 * t13988;
    let t14340 = t2409 * t8734 * t4097;
    let t14345 = t9270 * t4099;
    let t14351 = t2409 * t6781 * t4088;
    let t14358 = 7.0 / 144.0 * t14333 + 7.0 / 1152.0 * t13973 - t13977 / 48.0 - t13985 / 24.0 + t14338 + t3066 * t14340 / 24.0 + 7.0 / 36.0 * t14002 + 7.0 / 288.0 * t14114 - 7.0 / 72.0 * t14345 + t14119 / 768.0 + t14123 / 8.0 - 7.0 / 72.0 * t14128 + t2408 * t14351 / 24.0 - 7.0 / 72.0 * t14130 - t14133 / 768.0 - t14139 / 48.0 + t14141 / 48.0;
    (t14333, t14338, t14340, t14345, t14351, t14358)
}
