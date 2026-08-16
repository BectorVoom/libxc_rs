//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1030/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1030(t10428: f64, t275: f64, t1227: f64, t3730: f64, t921: f64, t2381: f64, t3757: f64, t6366: f64, t11153: f64, t179: f64, t932: f64, t2370: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11383 = t10428 * t275;
    let t11390 = t3730 * t1227 * t921;
    let t11391 = t2381 * t11390;
    let t11395 = t3757 * t1227 * t921;
    let t11396 = t6366 * t11395;
    let t11401 = t179 * t932 * t11153;
    let t11404 = t2370 * t1227;
    (t11383, t11390, t11391, t11395, t11396, t11401, t11404)
}
