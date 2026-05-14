//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 776/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk776<F: Float>(t6421: F, t6441: F, t6453: F, t6574: F, t105: F, t469: F, t96: F, t1298: F, t1670: F, t694: F, t1717: F, t814: F, t301: F, t1675: F, t1674: F, t1941: F) -> (F, F, F, F, F, F, F, F) {
    let t6576 = t6421 + t6441 + t6453 + t6574;
    let t6579 = t96 * t105 * t6576 * t469;
    let t6581 = t694 * t1670 * t1298;
    let t6583 = t1717 * t814;
    let t6585 = t694 * t6583 * t301;
    let t6589 = t1675 * t1298;
    let t6590 = t1674 * t6589;
    let t6592 = t1941 * t469;
    (t6576, t6579, t6581, t6583, t6585, t6589, t6590, t6592)
}
