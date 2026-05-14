//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 726/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk726<F: Float>(t1397: F, t7274: F, t913: F, t1405: F, t7878: F, t940: F, t141: F, t872: F, t2811: F, t3906: F, t7448: F, t2269: F, t2641: F, t11325: F, t3916: F, t322: F, t8425: F) -> (F, F, F, F, F, F, F) {
    let t11458 = t7274 * t1397;
    let t11459 = t913 * t11458;
    let t11469 = t7878 * t1405;
    let t11470 = t940 * t11469;
    let t11472 = t872 * t141;
    let t11473 = t2811 * t11472;
    let t11493 = t3906 * t7448;
    let t11518 = t2641 * t2269;
    let t11526 = t3916 * t11325;
    let t11596 = t322 * t8425;
    (t11459, t11470, t11473, t11493, t11518, t11526, t11596)
}
