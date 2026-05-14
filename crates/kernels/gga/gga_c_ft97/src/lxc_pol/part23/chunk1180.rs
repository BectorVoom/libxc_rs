//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1180/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1180<F: Float>(t113566: F, t113568: F, t113594: F, t113601: F, t113609: F, t113631: F, t113633: F, t1882: F, t29166: F, t29101: F, t10261: F, t871: F, t28924: F, t870: F, t29396: F, t29290: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t114453 = 2.0 / 27.0 * t113566;
    let t114454 = 2.0 / 81.0 * t113568;
    let t114465 = 4.0 / 9.0 * t113594;
    let t114467 = 4.0 / 9.0 * t113601;
    let t114469 = 4.0 / 9.0 * t113609;
    let t114482 = 4.0 / 27.0 * t113631;
    let t114483 = 4.0 / 27.0 * t113633;
    let t114499 = 2.0 / 27.0 * t1882 * t29166;
    let t114509 = 2.0 / 9.0 * t1882 * t29101;
    let t114531 = t10261 * t871;
    let t114554 = t870 * t28924;
    let t114565 = 2.0 / 9.0 * t1882 * t29396;
    let t114567 = 4.0 / 9.0 * t1882 * t29290;
    (t114453, t114454, t114465, t114467, t114469, t114482, t114483, t114499, t114509, t114531, t114554, t114565, t114567)
}
