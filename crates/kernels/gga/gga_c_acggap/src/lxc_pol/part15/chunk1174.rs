//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1174/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1174<F: Float>(t2001: F, t5975: F, t1846: F, t7685: F, t1817: F, t31811: F, t2030: F, t301: F, t8927: F, t9552: F, t2060: F, t36222: F, t372: F) -> (F, F, F, F, F) {
    let t40326 = t2001 * t5975;
    let t40330 = t7685 * t1846;
    let t40332 = t31811 * t1817;
    let t40336 = t2030 * t8927 * t9552 * t301;
    let t40340 = t2060 * t36222 * t9552 * t372;
    (t40326, t40330, t40332, t40336, t40340)
}
