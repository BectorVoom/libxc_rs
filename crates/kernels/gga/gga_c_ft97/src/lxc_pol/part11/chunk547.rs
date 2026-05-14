//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 547/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk547<F: Float>(t378: F, t7966: F, t92: F, t7789: F, t358: F, t7745: F) -> (F, F, F, F, F) {
    let t7967 = t378 * t7966;
    let t7968 = t92 * t7967;
    let t7970 = t378 * t7789;
    let t7971 = t92 * t7970;
    let t7973 = t358 * t7745;
    (t7967, t7968, t7970, t7971, t7973)
}
