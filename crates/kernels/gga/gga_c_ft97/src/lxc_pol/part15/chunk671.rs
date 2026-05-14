//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 671/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk671<F: Float>(t1053: F, t4668: F, t2185: F, t605: F, t1017: F, t4714: F, t167: F, t3578: F, t4733: F, t574: F, t4805: F, t4724: F, t2179: F, t20027: F, t2205: F, t12617: F, t16969: F, t1901: F, t20685: F, t20690: F, t20694: F, t20698: F, t20702: F, t20706: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20709 = t4668 * t1053;
    let t20711 = t2185 * t605 * t20709;
    let t20714 = t1017 * t4714;
    let t20716 = t2185 * t167 * t20714;
    let t20720 = t574 * t3578 * t4733;
    let t20723 = t4714 * t1053;
    let t20725 = t574 * t605 * t20723;
    let t20727 = t1017 * t4805;
    let t20729 = t574 * t605 * t20727;
    let t20731 = t4724 * t1017;
    let t20733 = t574 * t2179 * t20731;
    let t20737 = t2205 * t167 * t20027;
    let t20741 = t1901 * t20685 / 3.0 - 4.0 / 27.0 * t12617 - t446 * t20690 / 3.0 - 2.0 / 9.0 * t446 * t20694 - t446 * t20698 / 9.0 - 10.0 / 81.0 * t446 * t20702 - t446 * t20706 / 3.0 - 2.0 * t446 * t20711 + 2.0 * t446 * t20716 + 2.0 * t446 * t20720 + t446 * t20725 + t446 * t20729 - 2.0 * t446 * t20733 + 4.0 / 9.0 * t446 * t20737 - 2.0 / 3.0 * t16969;
    (t20709, t20711, t20714, t20716, t20720, t20723, t20725, t20727, t20729, t20731, t20733, t20737, t20741)
}
