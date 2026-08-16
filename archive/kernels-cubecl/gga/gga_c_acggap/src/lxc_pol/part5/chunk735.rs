//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 735/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk735<F: Float>(t1679: F, t467: F, t5399: F, t1662: F, t469: F, t301: F, t694: F, t1298: F, t192: F, t1674: F, t922: F, t495: F, t96: F) -> (F, F, F, F, F, F) {
    let t5401 = t1679 * t5399 * t467;
    let t5403 = t1662 * t469;
    let t5405 = t694 * t5403 * t301;
    let t5407 = t192 * t1298;
    let t5409 = t1674 * t5407 * t301;
    let t5412 = t922 * t192;
    let t5414 = t96 * t5412 * t495;
    (t5401, t5403, t5405, t5409, t5412, t5414)
}
