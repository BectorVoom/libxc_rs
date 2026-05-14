//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1003/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1003<F: Float>(t20251: F, t3065: F, t858: F, t6678: F, t6679: F, t9246: F, t6183: F, t6647: F, t6645: F, t20221: F, t20222: F, t20228: F, t20234: F, t20236: F, t20246: F, t20249: F, t2253: F, t2255: F, t2257: F, t2278: F, t2312: F, t3257: F, t6619: F, t851: F) -> (F, F, F, F) {
    let t20253 = t3065 * t858 * t20251;
    let t20255 = t6678 * t20253 / 24.0;
    let t20256 = t9246 * t6679;
    let t20257 = t6678 * t20256;
    let t20258 = 7.0 / 24.0 * t20257;
    let t20259 = t6183 * t6647;
    let t20260 = t6645 * t20259;
    let t20261 = 7.0 / 12.0 * t20260;
    let t20262 = t20221 - t2312 * t3257 * t2278 * t20222 / 16.0 + t20228 - t2253 * t2255 * t6619 * t2257 / 192.0 - t20234 - t2253 * t2255 * t851 * t20236 / 128.0 - t20246 - t20249 + t20255 - t20258 - t20261;
    (t20255, t20258, t20261, t20262)
}
