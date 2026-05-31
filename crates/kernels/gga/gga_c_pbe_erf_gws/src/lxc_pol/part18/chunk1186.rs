//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1186/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1186<F: Float>(t14031: F, t3765: F, t3810: F, t4039: F, t11628: F, t3139: F, t4028: F, t14073: F, t14085: F, t15070: F, t15072: F, t15074: F, t15076: F, t15249: F, t15251: F, t15253: F, t15256: F, t15258: F, t15260: F, t15262: F) -> (F, F) {
    let t15264 = t14031 * t3765;
    let t15266 = t4039 * t3810;
    let t15268 = t3139 * t11628;
    let t15269 = t4028 * t15268;
    let t15271 = -t15249 / F::cast_from(96.0_f64) - t15251 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t15253 + t15256 / F::cast_from(48.0_f64) + t15258 / F::cast_from(16.0_f64) - t15070 - t15260 / F::cast_from(48.0_f64) + t15262 / F::cast_from(96.0_f64) + t15072 + t15074 + t15076 + t14073 + t14085 - t15264 / F::cast_from(192.0_f64) + t15266 / F::cast_from(384.0_f64) - t15269 / F::cast_from(96.0_f64);
    (t15268, t15271)
}
