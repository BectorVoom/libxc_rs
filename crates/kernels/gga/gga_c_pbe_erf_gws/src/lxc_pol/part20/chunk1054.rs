//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1054/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1054<F: Float>(t11628: F, t3139: F, t4028: F, t14073: F, t14085: F, t15070: F, t15072: F, t15074: F, t15076: F, t15249: F, t15251: F, t15253: F, t15256: F, t15258: F, t15260: F, t15262: F, t15264: F, t15266: F) -> (F, F) {
    let t15268 = t3139 * t11628;
    let t15269 = t4028 * t15268;
    let t15271 = -t15249 / 96.0 - t15251 / 384.0 + 5.0 / 384.0 * t15253 + t15256 / 48.0 + t15258 / 16.0 - t15070 - t15260 / 48.0 + t15262 / 96.0 + t15072 + t15074 + t15076 + t14073 + t14085 - t15264 / 192.0 + t15266 / 384.0 - t15269 / 96.0;
    (t15268, t15271)
}
