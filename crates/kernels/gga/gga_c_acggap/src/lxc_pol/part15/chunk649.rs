//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 649/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk649<F: Float>(t1137: F, t1867: F, t145: F, t1713: F, t301: F, t960: F, t1884: F, t372: F, t1298: F, t1313: F, t1734: F, t1753: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6371 = t1137 * t1867;
    let t6374 = t145 * t1713;
    let t6375 = t6374 * t301;
    let t6376 = t960 * t6375;
    let t6379 = t1884 * t372;
    let t6380 = t960 * t6379;
    let t6383 = t1313 * t1298;
    let t6384 = t960 * t6383;
    let t6387 = t145 * t1734;
    let t6388 = t6387 * t301;
    let t6389 = t960 * t6388;
    let t6394 = t1753 * t372;
    (t6371, t6375, t6376, t6379, t6380, t6383, t6384, t6388, t6389, t6394)
}
