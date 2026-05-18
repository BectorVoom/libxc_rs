//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 848/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk848<F: Float>(t1345: F, t322: F, t1662: F, t301: F, t467: F, t495: F, t811: F, t694: F, t7298: F, t104: F, t8020: F, t2162: F, t469: F) -> (F, F, F, F, F, F, F) {
    let t23745 = t1345 * t322;
    let t24589 = t301 * t1662;
    let t24605 = t1662 * t467;
    let t24623 = t495 * t811;
    let t29938 = t694 * t7298;
    let t29943 = t104 * t8020;
    let t29948 = t2162 * t469;
    (t23745, t24589, t24605, t24623, t29938, t29943, t29948)
}
