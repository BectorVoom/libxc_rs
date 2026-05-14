//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 940/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk940<F: Float>(t1800: F, t29706: F, t1317: F, t28: F, t1307: F, t4533: F, t469: F, t5665: F, t6454: F, t965: F, t4436: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29707 = t1800 * t29706;
    let t29709 = t1317 * t28 * t29707;
    let t29711 = t1307 * t4533;
    let t29712 = t469 * t29711;
    let t29714 = t5665 * t28 * t29712;
    let t29716 = t6454 * t965;
    let t29717 = t469 * t29716;
    let t29719 = t5665 * t28 * t29717;
    let t29721 = t1307 * t4436;
    (t29707, t29709, t29711, t29712, t29714, t29716, t29717, t29719, t29721)
}
