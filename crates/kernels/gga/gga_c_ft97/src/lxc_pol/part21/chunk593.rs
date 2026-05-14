//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 593/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk593<F: Float>(t26: F, t356: F, t1570: F, t469: F, t11069: F, t11076: F, t11416: F, t100: F, t1587: F) -> (F, F, F, F, F, F) {
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11778 = 2.0 / 9.0 * t11069;
    let t11781 = 4.0 / 27.0 * t11076;
    let t11798 = 4.0 / 9.0 * t11416;
    let t11810 = t1587 * t100;
    (t11761, t11762, t11778, t11781, t11798, t11810)
}
