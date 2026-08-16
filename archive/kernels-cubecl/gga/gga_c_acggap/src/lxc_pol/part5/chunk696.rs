//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 696/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk696<F: Float>(t157: F, t922: F, t1165: F, t1532: F, t4183: F, t3451: F, t1541: F, t3372: F, t1298: F, t372: F, t1089: F, t1095: F) -> (F, F, F, F, F, F, F) {
    let t4919 = t157 * t922;
    let t4921 = t1165 * t1532 * t4919;
    let t4925 = t1165 * t1532 * t4183;
    let t4926 = t3451 * t4925;
    let t4928 = t3372 * t1541;
    let t4930 = t1298 * t372;
    let t4932 = t1089 * t1095 * t4930;
    (t4919, t4921, t4925, t4926, t4928, t4930, t4932)
}
