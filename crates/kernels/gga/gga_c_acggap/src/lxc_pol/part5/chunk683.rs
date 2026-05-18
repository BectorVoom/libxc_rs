//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 683/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk683<F: Float>(t1501: F, t336: F, t839: F, t1579: F, t372: F, t1143: F, t1298: F, t337: F, t4099: F, t1137: F, t1503: F, t3565: F, t495: F) -> (F, F, F, F, F, F) {
    let t4769 = t336 * t1501 * t839;
    let t4773 = t336 * t1579 * t372;
    let t4777 = t336 * t1143 * t1298;
    let t4781 = t336 * t337 * t4099;
    let t4785 = F::new(7.0) / F::new(72.0) * t1137 * t1503;
    let t4787 = t336 * t3565 * t495;
    (t4769, t4773, t4777, t4781, t4785, t4787)
}
