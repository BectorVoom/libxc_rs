//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 938/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk938<F: Float>(t29692: F, t469: F, t23009: F, t28: F, t29641: F, t7824: F, t446: F, t1307: F, t4495: F) -> (F, F, F, F, F) {
    let t29693 = t469 * t29692;
    let t29695 = t23009 * t28 * t29693;
    let t29698 = t7824 * t29641;
    let t29699 = t446 * t29698;
    let t29701 = t1307 * t4495;
    (t29693, t29695, t29698, t29699, t29701)
}
