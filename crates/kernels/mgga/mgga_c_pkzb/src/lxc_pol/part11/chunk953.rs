//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 953/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk953<F: Float>(t10428: F, t275: F, t1227: F, t3730: F, t921: F, t2381: F, t3757: F, t6366: F, t11153: F, t179: F, t932: F, t2370: F, t10083: F, t406: F, t3898: F, t3913: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11383 = t10428 * t275;
    let t11390 = t3730 * t1227 * t921;
    let t11391 = t2381 * t11390;
    let t11395 = t3757 * t1227 * t921;
    let t11396 = t6366 * t11395;
    let t11401 = t179 * t932 * t11153;
    let t11404 = t2370 * t1227;
    let t11405 = t10083 * t11404;
    let t11406 = t406 * t11405;
    let t11409 = t3913 * t3898;
    (t11383, t11390, t11391, t11395, t11396, t11401, t11404, t11405, t11406, t11409)
}
