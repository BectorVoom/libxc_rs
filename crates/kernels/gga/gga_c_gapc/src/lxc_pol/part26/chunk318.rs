//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 318/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk318<F: Float>(t131: F, t20: F, t1354: F, t14: F, t70: F, t543: F, t402: F, t78: F, t4: F, t3: F, t95: F, t545: F) -> (F, F, F, F, F, F) {
    let t1355 = t20 * t131;
    let t1356 = t1354 * t1355;
    let t1359 = t14 * t70;
    let t1360 = t543 * t1359;
    let t1361 = t78 * t402;
    let t1362 = t4 * t1361;
    let t1365 = t3 * t95;
    let t1366 = t545 * t1365;
    (t1355, t1356, t1360, t1362, t1365, t1366)
}
