//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 781/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk781<F: Float>(t1854: F, t322: F, t406: F, t7158: F, t372: F, t1298: F, t525: F, t301: F, t1016: F, t1742: F, t1662: F, t495: F, t2162: F, t469: F, t7927: F, t880: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26554 = t1854 * t322;
    let t26757 = t7158 * t406;
    let t26956 = t1854 * t372;
    let t26995 = t525 * t1298;
    let t27011 = t1854 * t301;
    let t27338 = t1742 * t1016;
    let t28242 = t495 * t1662;
    let t29948 = t2162 * t469;
    let t29973 = 0.19756347548806534796e1 * t7927 * t880;
    (t26554, t26757, t26956, t26995, t27011, t27338, t28242, t29948, t29973)
}
