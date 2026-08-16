//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 845/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk845<F: Float>(t1555: F, t1558: F, t37362: F, t89: F, t1546: F, t7784: F, t356: F, t37357: F, t7801: F, t1571: F, t7780: F, t7802: F) -> (F, F, F, F, F) {
    let t37365 = t89 * t1555 * t1558 * t37362;
    let t37368 = t89 * t1546 * t7784;
    let t37372 = t89 * t356 * t7801 * t37357;
    let t37376 = t89 * t356 * t1571 * t37362;
    let t37379 = t89 * t7780 * t7802;
    (t37365, t37368, t37372, t37376, t37379)
}
