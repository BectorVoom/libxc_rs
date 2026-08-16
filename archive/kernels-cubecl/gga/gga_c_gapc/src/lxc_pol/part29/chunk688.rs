//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 688/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk688<F: Float>(t332: F, t918: F, t2776: F, t442: F, t2642: F, t959: F, t2206: F, t871: F) -> (F, F, F, F) {
    let t7418 = t918 * t332;
    let t7419 = t2776 * t442;
    let t7420 = t7418 * t7419;
    let t7442 = t2642 * t959 * t332;
    let t7451 = t871 * t2206;
    (t7418, t7420, t7442, t7451)
}
