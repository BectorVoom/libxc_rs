//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1128/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1128<F: Float>(t20259: F, t6645: F, t20221: F, t20222: F, t20228: F, t20234: F, t20236: F, t20246: F, t20249: F, t20255: F, t20258: F, t2253: F, t2255: F, t2257: F, t2278: F, t2312: F, t3257: F, t6619: F, t851: F) -> (F, F) {
    let t20260 = t6645 * t20259;
    let t20261 = F::new(7.0) / F::new(12.0) * t20260;
    let t20262 = t20221 - t2312 * t3257 * t2278 * t20222 / F::new(16.0) + t20228 - t2253 * t2255 * t6619 * t2257 / F::new(192.0) - t20234 - t2253 * t2255 * t851 * t20236 / F::new(128.0) - t20246 - t20249 + t20255 - t20258 - t20261;
    (t20261, t20262)
}
