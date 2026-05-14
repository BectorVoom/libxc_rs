//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 623/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk623<F: Float>(t4054: F, t1552: F, t435: F, t128: F, t505: F, t188: F, t516: F, t424: F, t515: F, t3668: F, t653: F, t1870: F, t442: F, t1431: F, t1631: F, t1463: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4055 = t4054 * M_PI;
    let t4059 = t435 * t1552;
    let t4296 = t128 * t505;
    let t4533 = t516 * t188;
    let t4538 = t424 * t515;
    let t4605 = t3668 * t653;
    let t4644 = t1870 * t442;
    let t4687 = t1431 * t442;
    let t4780 = t1631 * t128;
    let t4855 = t1463 * t442;
    (t4055, t4059, t4296, t4533, t4538, t4605, t4644, t4687, t4780, t4855)
}
