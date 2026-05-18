//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 727/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk727<F: Float>(t20573: F, t20676: F, t515: F, t16963: F, t925: F, t2221: F, t1060: F, t4462: F, t569: F, t2205: F, t4454: F, t167: F, t20045: F) -> (F, F, F, F, F, F, F) {
    let t20677 = t20573 + t20676;
    let t20678 = t515 * t20677;
    let t20684 = t16963 * t925;
    let t20685 = t2221 * t20684;
    let t20690 = t569 * t1060 * t4462;
    let t20694 = t2205 * t1060 * t4454;
    let t20698 = t569 * t167 * t20045;
    (t20677, t20678, t20684, t20685, t20690, t20694, t20698)
}
