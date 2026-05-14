//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 278/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk278<F: Float>(t1273: F, t332: F, t113: F, t6: F, t695: F, t224: F, t817: F, t285: F) -> (F, F, F, F, F, F) {
    let t1274 = t1273 * t332;
    let t1275 = t1274 * t113;
    let t1416 = t695 * t6;
    let t1417 = t224 * t1416;
    let t1471 = t817 * t6;
    let t1472 = t285 * t1471;
    (t1274, t1275, t1416, t1417, t1471, t1472)
}
