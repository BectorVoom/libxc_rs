//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 647/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk647<F: Float>(t1016: F, t506: F, t1460: F, t1181: F, t1165: F, t1884: F, t407: F, t1350: F, t530: F, t3361: F, t1539: F, t5862: F) -> (F, F, F, F, F) {
    let t6337 = t1016 * t506;
    let t6338 = t6337 * t1460;
    let t6339 = t1181 * t6338;
    let t6343 = t1165 * t1884 * t407;
    let t6346 = t530 * t1350;
    let t6347 = t1181 * t6346;
    let t6348 = t3361 * t6347;
    let t6351 = t1165 * t5862 * t1539;
    (t6339, t6343, t6347, t6348, t6351)
}
