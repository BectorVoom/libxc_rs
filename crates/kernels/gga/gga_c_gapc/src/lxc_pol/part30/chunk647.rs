//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 647/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk647<F: Float>(t4054: F, t1552: F, t435: F, t128: F, t505: F, t188: F, t516: F, t424: F, t515: F, t3668: F, t653: F, t1870: F, t442: F) -> (F, F, F, F, F, F, F) {
    let t4055 = t4054 * M_PI;
    let t4059 = t435 * t1552;
    let t4296 = t128 * t505;
    let t4533 = t516 * t188;
    let t4538 = t424 * t515;
    let t4605 = t3668 * t653;
    let t4644 = t1870 * t442;
    (t4055, t4059, t4296, t4533, t4538, t4605, t4644)
}
