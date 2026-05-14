//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 508/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk508<F: Float>(t3237: F, t431: F, t438: F, t314: F, t847: F, t150: F, t383: F, t390: F, t336: F, t360: F, t1016: F, t141: F) -> (F, F, F, F, F, F, F) {
    let t3238 = t3237 * t431;
    let t3240 = t3237 * t438;
    let t3242 = t847 * t314;
    let t3243 = t3242 * t150;
    let t3244 = t3243 * t383;
    let t3246 = 0.64311027177104605458e-3 * t3244 * t390;
    let t3282 = t336 * t360;
    let t3300 = t141 * t1016;
    (t3238, t3240, t3242, t3243, t3246, t3282, t3300)
}
