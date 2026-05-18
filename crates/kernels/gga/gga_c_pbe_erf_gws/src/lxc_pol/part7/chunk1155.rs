//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1155/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1155<F: Float>(t20651: F, t6567: F, t2147: F, t337: F, t6340: F, t810: F, t6339: F, t6211: F, t814: F, t19561: F, t20623: F, t20626: F, t20631: F, t20638: F, t20640: F, t20647: F, t20649: F, t2190: F, t2255: F, t2277: F, t2278: F, t2312: F, t2343: F, t6366: F, t6367: F, t6470: F, t9482: F) -> (F, F, F) {
    let t20652 = t6567 * t20651;
    let t20653 = F::new(7.0) / F::new(36.0) * t20652;
    let t20656 = t2147 * t337 * t6340 * t810;
    let t20658 = t6339 * t20656 / F::new(4.0);
    let t20659 = t6211 * t814;
    let t20664 = -t20623 + F::new(7.0) / F::new(48.0) * t20626 + t20631 - F::new(5.0) / F::new(64.0) * t2343 * t6366 * t6367 * t2190 + t20638 - t2277 * t9482 * t6470 * t19561 * t20640 / F::new(64.0) + F::new(595.0) / F::new(1296.0) * t20647 + F::new(7.0) / F::new(96.0) * t20649 - t20653 - t20658 + t2312 * t2255 * t2278 * t20659 / F::new(48.0);
    (t20653, t20658, t20664)
}
