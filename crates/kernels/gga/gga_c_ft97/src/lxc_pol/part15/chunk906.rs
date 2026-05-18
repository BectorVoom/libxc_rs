//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 906/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk906<F: Float>(t4617: F, t8232: F, t1637: F, t4547: F, t89: F, t4553: F, t1570: F, t1851: F, t1557: F, t4603: F, t4574: F, t4565: F) -> (F, F, F, F, F, F, F, F) {
    let t59801 = t8232 * t4617;
    let t59838 = t89 * t1637 * t4547;
    let t59937 = t8232 * t4553;
    let t60031 = t1851 * t1570;
    let t60100 = t1851 * t1557;
    let t60151 = t8232 * t4603;
    let t60309 = t8232 * t4574;
    let t60358 = t8232 * t4565;
    (t59801, t59838, t59937, t60031, t60100, t60151, t60309, t60358)
}
