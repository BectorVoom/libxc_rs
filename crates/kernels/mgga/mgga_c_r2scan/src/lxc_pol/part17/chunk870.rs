//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 870/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk870<F: Float>(t11031: F, t11057: F, t3506: F, t833: F, t1120: F, t1299: F, t1338: F, t3552: F, t1142: F, t6755: F, t1348: F, t6767: F, t1561: F, t3492: F, t1114: F, t2333: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11206 = 22.0 / 9.0 * t11031;
    let t11215 = 22.0 / 9.0 * t11057;
    let t11220 = t3506 * t833;
    let t11223 = t1120 * t1299;
    let t11302 = t1338 * t3552;
    let t11305 = t6755 * t1142;
    let t11314 = t1348 * t3552;
    let t11319 = t6767 * t1142;
    let t11325 = t1561 * t3492;
    let t11336 = t1114 * t2333;
    (t11206, t11215, t11220, t11223, t11302, t11305, t11314, t11319, t11325, t11336)
}
