//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 753/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk753<F: Float>(t22604: F, t3065: F, t39: F, t78: F, t5517: F, t66: F, t37: F, t401: F, t1299: F, t1664: F, t139: F, t527: F, t135: F, t1995: F, t1354: F, t7240: F, t81: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22605 = t3065 * t22604;
    let t22686 = t78 * t39;
    let t22696 = t5517 * t66;
    let t22833 = t37 * t401;
    let t22834 = t22833 * t78;
    let t22852 = t1664 * t1299;
    let t23809 = t139 * t39;
    let t23810 = t527 * t23809;
    let t23831 = t1995 * t135;
    let t23847 = t1995 * t1354;
    let t23866 = t1995 * t23809;
    let t23869 = t527 * t1354;
    let t32075 = 1.0 / t7240 / t81;
    (t22605, t22686, t22696, t22834, t22852, t23810, t23831, t23847, t23866, t23869, t32075)
}
