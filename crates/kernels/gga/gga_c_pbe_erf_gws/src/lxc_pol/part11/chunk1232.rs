//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1232/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1232<F: Float>(t1105: F, t1123: F, t13534: F, t13578: F, t2255: F, t2312: F, t2345: F, t3235: F, t3247: F, t3373: F, t36803: F, t3752: F, t37829: F, t44282: F, t44710: F, t49415: F, t49464: F, t49471: F, t49472: F, t49478: F, t49483: F, t824: F, t8884: F, t902: F, t905: F, t9425: F, t9482: F) -> F {
    let t49489 = -t2312 * t2255 * t1123 * t3373 * t1105 / F::new(96.0) - t2312 * t9482 * t13578 * t37829 / F::new(24.0) - t2312 * t2255 * t13534 * t3752 / F::new(96.0) + t49415 + t902 * t905 * t49464 * t824 / F::new(1536.0) - t49471 - t49472 - t49478 + t3247 * t3235 * t44710 * t8884 / F::new(128.0) + t9425 * t2345 * t44282 * t49483 / F::new(8.0) + F::new(119.0) / F::new(576.0) * t36803;
    t49489
}
