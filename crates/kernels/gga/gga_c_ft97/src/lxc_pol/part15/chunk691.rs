//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 691/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk691<F: Float>(t20123: F, t20161: F, t348: F, t4572: F, t925: F, t8557: F, t4436: F, t979: F, t1871: F, t488: F, t4495: F, t942: F) -> (F, F, F, F, F, F, F) {
    let t20162 = t20123 + t20161;
    let t20163 = t348 * t20162;
    let t20171 = t4572 * t925;
    let t20172 = t8557 * t20171;
    let t20177 = t4436 * t979;
    let t20179 = t1871 * t488 * t20177;
    let t20182 = t942 * t4495;
    (t20162, t20163, t20171, t20172, t20177, t20179, t20182)
}
