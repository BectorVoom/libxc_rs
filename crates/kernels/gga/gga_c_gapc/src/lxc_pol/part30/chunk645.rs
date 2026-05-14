//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 645/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk645<F: Float>(t786: F, t952: F, t3327: F, t7158: F, t875: F, t2761: F, t959: F, t967: F, t935: F, t4978: F, t961: F, t2801: F, t329: F, t332: F, t918: F, t2776: F, t442: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7324 = t952 * t786;
    let t7325 = t3327 * t7324;
    let t7329 = t7158 * t875;
    let t7330 = t3327 * t7329;
    let t7333 = t2761 * t959;
    let t7334 = t967 * t786;
    let t7335 = t7333 * t7334;
    let t7371 = t935 * t875;
    let t7375 = t961 * t4978;
    let t7389 = t329 * t2801;
    let t7418 = t918 * t332;
    let t7419 = t2776 * t442;
    (t7325, t7330, t7333, t7335, t7371, t7375, t7389, t7418, t7419)
}
