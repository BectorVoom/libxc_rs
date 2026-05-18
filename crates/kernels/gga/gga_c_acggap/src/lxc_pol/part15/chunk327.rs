//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 327/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk327<F: Float>(t1487: F, t5: F, t129: F, t145: F, t1143: F, t336: F, t495: F, t1298: F, t337: F, t506: F, t301: F, t372: F, t535: F) -> (F, F, F, F, F, F, F) {
    let t1488 = t5 * t1487;
    let t1490 = t129 * t1488 * t145;
    let t1494 = t336 * t1143 * t495;
    let t1498 = t336 * t337 * t1298;
    let t1501 = t506 * t145;
    let t1503 = t336 * t1501 * t301;
    let t1507 = t336 * t535 * t372;
    (t1488, t1490, t1494, t1498, t1501, t1503, t1507)
}
