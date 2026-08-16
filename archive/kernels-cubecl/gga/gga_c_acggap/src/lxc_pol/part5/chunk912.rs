//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 912/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk912<F: Float>(t1164: F, t3266: F, t1037: F, t1165: F, t3451: F, t955: F, t1084: F, t1181: F, t13232: F, t3391: F, t1111: F, t12936: F) -> (F, F, F, F) {
    let t13889 = t1164 * t3266;
    let t13911 = t3451 * t1165 * t1037 * t955;
    let t13915 = t3391 * t1181 * t13232 * t1084;
    let t13919 = t12936 * t1165 * t13232 * t1111;
    (t13889, t13911, t13915, t13919)
}
