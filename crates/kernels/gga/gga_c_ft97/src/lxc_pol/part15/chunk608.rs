//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 608/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk608<F: Float>(t8282: F, t959: F, t1555: F, t26: F, t1557: F, t469: F, t356: F, t1570: F, t1800: F, t942: F, t100: F, t1587: F) -> (F, F, F, F, F, F, F) {
    let t11720 = t8282 * t959;
    let t11755 = t26 * t1555;
    let t11756 = t469 * t1557;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11766 = t1800 * t942;
    let t11810 = t1587 * t100;
    (t11720, t11755, t11756, t11761, t11762, t11766, t11810)
}
