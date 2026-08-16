//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 669/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk669<F: Float>(t1314: F, t3282: F, t1318: F, t1298: F, t145: F, t301: F, t960: F, t1567: F, t372: F, t1131: F, t530: F, t1327: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4571 = t3282 * t1314;
    let t4574 = t3282 * t1318;
    let t4577 = t145 * t1298;
    let t4578 = t4577 * t301;
    let t4579 = t960 * t4578;
    let t4582 = t1567 * t372;
    let t4583 = t960 * t4582;
    let t4586 = t530 * t1131;
    let t4587 = t960 * t4586;
    let t4590 = t3282 * t1327;
    (t4571, t4574, t4578, t4579, t4582, t4583, t4586, t4587, t4590)
}
