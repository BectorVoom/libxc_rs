//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 647/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk647<F: Float>(t126: F, t4864: F, t102: F, t457: F, t1946: F, t1302: F, t515: F, t1709: F, t442: F, t1983: F, t1609: F, t575: F) -> (F, F, F, F, F, F, F, F) {
    let t4865 = t4864 * t126;
    let t4867 = t102 * t457;
    let t4868 = t1946 * t4867;
    let t4882 = t1302 * t515;
    let t4883 = t4882 * t126;
    let t4885 = t1709 * t442;
    let t4893 = t1983 * t442;
    let t4905 = t1609 * t575;
    (t4865, t4867, t4868, t4882, t4883, t4885, t4893, t4905)
}
