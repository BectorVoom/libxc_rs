//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 497/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk497<F: Float>(t245: F, t4363: F, t125: F, t1558: F, t1544: F, t854: F, t236: F, t807: F, t1469: F, t2375: F, t2382: F, t1532: F, t750: F) -> (F, F, F, F, F, F, F, F) {
    let t4364 = t4363 * t245;
    let t4365 = t125 * t1558;
    let t4371 = t854 * t1544;
    let t4372 = t236 * t4371;
    let t4373 = t807 * t4372;
    let t4377 = t2375 * t1469;
    let t4384 = t2382 * t1469;
    let t4397 = t1532 * t750;
    (t4364, t4365, t4371, t4372, t4373, t4377, t4384, t4397)
}
