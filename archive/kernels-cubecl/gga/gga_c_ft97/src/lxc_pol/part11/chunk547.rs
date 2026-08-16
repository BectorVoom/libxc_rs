//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 547/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk547<F: Float>(t7764: F, t7765: F, t7761: F, t89: F, t1589: F, t375: F, t1636: F, t355: F) -> (F, F, F, F) {
    let t7766 = t7764 * t7765;
    let t7768 = t89 * t7761 * t7766;
    let t7771 = t89 * t375 * t1589;
    let t7773 = t1636 * t355;
    (t7766, t7768, t7771, t7773)
}
