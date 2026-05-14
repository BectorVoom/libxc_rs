//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 317/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk317<F: Float>(t1338: F, t431: F, t1249: F, t159: F, t104: F, t405: F, t14: F, t445: F, t73: F, t348: F, t108: F, t19: F, t131: F, t20: F, t70: F, t543: F) -> (F, F, F, F, F, F, F) {
    let t1339 = t431 * t1338;
    let t1343 = t1249 * t159;
    let t1346 = t405 * t104;
    let t1347 = t1346 * t14;
    let t1352 = t73 * t445;
    let t1353 = t1352 * t348;
    let t1354 = t108 * t19;
    let t1355 = t20 * t131;
    let t1356 = t1354 * t1355;
    let t1359 = t14 * t70;
    let t1360 = t543 * t1359;
    (t1339, t1343, t1347, t1353, t1355, t1356, t1360)
}
