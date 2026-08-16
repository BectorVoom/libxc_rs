//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1134/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1134(t6398: f64, t6402: f64, t6331: f64, t6534: f64, t2120: f64, t11541: f64, t20306: f64, t20321: f64, t20322: f64, t20328: f64, t20335: f64, t20345: f64, t2113: f64, t2253: f64, t2255: f64, t2277: f64, t2312: f64, t3257: f64, t6297: f64, t6350: f64, t6396: f64, t6664: f64, t6685: f64, t851: f64, t9332: f64) -> (f64, f64) {
    let t20350 = t6402 * t6398;
    let t20355 = t6331 * t6534;
    let t20356 = t2120 * t20355;
    let t20357 = 7.0_f64 / 12.0_f64 * t20356;
    let t20358 = -t20321 - t2312 * t2255 * t6664 * t20322 / 48.0_f64 + t20328 - t2253 * t2255 * t2113 * t6297 / 96.0_f64 + t20335 - t2253 * t2255 * t2113 * t6396 / 128.0_f64 + 7.0_f64 / 384.0_f64 * t2277 * t3257 * t6350 * t9332 - t2253 * t2255 * t851 * t20345 / 128.0_f64 + 7.0_f64 / 96.0_f64 * t20350 + 3.0_f64 / 64.0_f64 * t6685 * t20306 * t11541 - t20357;
    (t20357, t20358)
}
