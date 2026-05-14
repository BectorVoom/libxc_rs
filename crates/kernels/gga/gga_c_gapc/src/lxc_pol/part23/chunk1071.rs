//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1071/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1071<F: Float>(t1026: F, t1845: F, t3018: F, t11391: F, t3022: F, t1803: F, t8738: F, t11594: F, t21838: F, t21631: F, t11397: F, t11402: F, t424: F, t11401: F, t3074: F, t35085: F) -> (F, F, F, F, F, F, F) {
    let t35334 = t1845 * t1026 * t3018;
    let t35336 = t11391 * t3022;
    let t35339 = t1803 * t1026 * t8738;
    let t35341 = t11594 * t21838;
    let t35343 = t11594 * t21631;
    let t35346 = t424 * t11397 * t11402;
    let t35348 = t3074 * t11401;
    let t35349 = t35085 * t35348;
    (t35334, t35336, t35339, t35341, t35343, t35346, t35349)
}
