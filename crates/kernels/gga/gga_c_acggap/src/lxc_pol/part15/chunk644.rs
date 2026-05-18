//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 644/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk644<F: Float>(t301: F, t6288: F, t960: F, t1899: F, t372: F, t1866: F, t3282: F, t1567: F, t513: F, t1524: F, t530: F, t1782: F, t435: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6289 = t6288 * t301;
    let t6290 = t960 * t6289;
    let t6293 = t1899 * t372;
    let t6294 = t960 * t6293;
    let t6297 = t3282 * t1866;
    let t6300 = t1567 * t513;
    let t6301 = t960 * t6300;
    let t6304 = t530 * t1524;
    let t6305 = t960 * t6304;
    let t6308 = t435 * t1782;
    (t6289, t6290, t6293, t6294, t6297, t6300, t6301, t6304, t6305, t6308)
}
