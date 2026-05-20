//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 787/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk787<F: Float>(t1298: F, t498: F, t1300: F, t198: F, t336: F, t3378: F, t3381: F, t3388: F, t3430: F, t3438: F, t3528: F, t3530: F, t3533: F, t3537: F, t3541: F, t3545: F, t3794: F) -> (F, F, F, F) {
    let t3798 = t1298 * t1298;
    let t3800 = t498 * t498;
    let t3801 = F::new(1.0) / t3800;
    let t3804 = t1300 * t198 * t336 * t3794 - t198 * t336 * t3798 * t3801 - t3378 + t3381 - t3388 + t3430 + t3438 + t3528 + t3530 - t3533 + t3537 - t3541 - t3545;
    (t3798, t3800, t3801, t3804)
}
