//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1001/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1001<F: Float>(t33293: F, t33294: F, t3628: F, t3746: F, t6108: F, t27805: F, t33319: F, t9770: F, t33476: F, t3875: F, t446: F, t1434: F, t35339: F, t681: F) -> (F, F, F, F, F) {
    let t150114 = t6108 * t3628 * t33293 * t33294 * t3746;
    let t150118 = t27805 * t9770 * t33319 * t3746;
    let t150120 = t33476 * t3875;
    let t150122 = t446 * t9770 * t150120;
    let t150125 = t1434 * t681 * t35339;
    (t150114, t150118, t150120, t150122, t150125)
}
