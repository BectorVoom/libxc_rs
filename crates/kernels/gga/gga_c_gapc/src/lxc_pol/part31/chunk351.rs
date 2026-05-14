//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 351/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk351<F: Float>(t1411: F, t1480: F, t1571: F, t1607: F, t572: F, t575: F, t208: F, t574: F) -> (F, F, F) {
    let t1609 = t1411 + t1480 + t1571 + t1607;
    let t1611 = t572 * t575;
    let t1615 = 1.0 / t574 / t208;
    (t1609, t1611, t1615)
}
