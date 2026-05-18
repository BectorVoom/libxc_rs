//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1140/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1140<F: Float>(t6183: F, t6325: F, t6327: F, t346: F, t6472: F, t6800: F, t2150: F, t6702: F, t6707: F, t20400: F, t20401: F, t20403: F, t20410: F, t20414: F, t20416: F, t20424: F, t20428: F, t2253: F, t2277: F, t3257: F, t6195: F, t6609: F, t9482: F) -> (F, F, F, F, F) {
    let t20430 = t6325 * t6183 * t6327;
    let t20431 = F::new(7.0) / F::new(24.0) * t20430;
    let t20432 = t6472 * t346;
    let t20433 = t6800 * t20432;
    let t20435 = t20433 * t2150 / F::new(12.0);
    let t20437 = t6702 * t6707 / F::new(32.0);
    let t20438 = -t20400 + F::new(7.0) / F::new(96.0) * t20401 - F::new(7.0) / F::new(384.0) * t2277 * t3257 * t6195 * t20403 + t20410 + t20414 - t2253 * t9482 * t6609 * t20416 / F::new(48.0) + t20424 + t20428 - t20431 - t20435 - t20437;
    (t20431, t20432, t20435, t20437, t20438)
}
