//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1155/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1155(t20651: f64, t6567: f64, t2147: f64, t337: f64, t6340: f64, t810: f64, t6339: f64, t6211: f64, t814: f64, t19561: f64, t20623: f64, t20626: f64, t20631: f64, t20638: f64, t20640: f64, t20647: f64, t20649: f64, t2190: f64, t2255: f64, t2277: f64, t2278: f64, t2312: f64, t2343: f64, t6366: f64, t6367: f64, t6470: f64, t9482: f64) -> (f64, f64, f64) {
    let t20652 = t6567 * t20651;
    let t20653 = 7.0_f64 / 36.0_f64 * t20652;
    let t20656 = t2147 * t337 * t6340 * t810;
    let t20658 = t6339 * t20656 / 4.0_f64;
    let t20659 = t6211 * t814;
    let t20664 = -t20623 + 7.0_f64 / 48.0_f64 * t20626 + t20631 - 5.0_f64 / 64.0_f64 * t2343 * t6366 * t6367 * t2190 + t20638 - t2277 * t9482 * t6470 * t19561 * t20640 / 64.0_f64 + 595.0_f64 / 1296.0_f64 * t20647 + 7.0_f64 / 96.0_f64 * t20649 - t20653 - t20658 + t2312 * t2255 * t2278 * t20659 / 48.0_f64;
    (t20653, t20658, t20664)
}
