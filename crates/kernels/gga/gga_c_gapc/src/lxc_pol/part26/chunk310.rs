//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 310/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk310<F: Float>(t118: F, t1266: F, t61: F, t119: F, t482: F, t101: F, t132: F, t433: F, t472: F, t78: F, t423: F, t9: F, t1265: F, t22: F, t114: F, t430: F) -> (F, F, F, F, F, F, F, F) {
    let t1267 = t1266 * t118;
    let t1268 = t61 * t1267;
    let t1273 = t482 * t119;
    let t1276 = t132 * t101;
    let t1277 = t1276 * t433;
    let t1280 = t1276 * t472;
    let t1283 = t78 * t101;
    let t1287 = t9 * t423;
    let t1294 = t22 * t1265;
    let t1302 = 1.0 / t430 / t114;
    (t1268, t1273, t1277, t1280, t1283, t1287, t1294, t1302)
}
