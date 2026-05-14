//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 739/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk739<F: Float>(t11175: F, t9: F, t534: F, t7858: F, t371: F, t7876: F, t25: F, t78: F, t1602: F, t122: F, t173: F, t1736: F, t420: F, t8119: F, t401: F, t428: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11176 = t9 * t11175;
    let t11209 = t534 * t7858;
    let t11232 = t371 * t7876;
    let t11240 = t78 * t25;
    let t11241 = t1602 * t11240;
    let t11245 = t78 * t122;
    let t11246 = t1602 * t11245;
    let t11262 = t173 * t1736;
    let t11269 = t420 * t8119;
    let t11335 = t401 * t428;
    (t11176, t11209, t11232, t11240, t11241, t11246, t11262, t11269, t11335)
}
