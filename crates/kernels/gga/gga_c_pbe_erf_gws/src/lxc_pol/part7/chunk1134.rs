//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1134/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1134<F: Float>(t6398: F, t6402: F, t6331: F, t6534: F, t2120: F, t11541: F, t20306: F, t20321: F, t20322: F, t20328: F, t20335: F, t20345: F, t2113: F, t2253: F, t2255: F, t2277: F, t2312: F, t3257: F, t6297: F, t6350: F, t6396: F, t6664: F, t6685: F, t851: F, t9332: F) -> (F, F) {
    let t20350 = t6402 * t6398;
    let t20355 = t6331 * t6534;
    let t20356 = t2120 * t20355;
    let t20357 = F::new(7.0) / F::new(12.0) * t20356;
    let t20358 = -t20321 - t2312 * t2255 * t6664 * t20322 / F::new(48.0) + t20328 - t2253 * t2255 * t2113 * t6297 / F::new(96.0) + t20335 - t2253 * t2255 * t2113 * t6396 / F::new(128.0) + F::new(7.0) / F::new(384.0) * t2277 * t3257 * t6350 * t9332 - t2253 * t2255 * t851 * t20345 / F::new(128.0) + F::new(7.0) / F::new(96.0) * t20350 + F::new(3.0) / F::new(64.0) * t6685 * t20306 * t11541 - t20357;
    (t20357, t20358)
}
