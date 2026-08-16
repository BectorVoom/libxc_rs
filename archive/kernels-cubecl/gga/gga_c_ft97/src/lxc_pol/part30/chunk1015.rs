//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1015/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1015<F: Float>(t141369: F, t35315: F, t140757: F, t140762: F, t140833: F, t27845: F, t6817: F, t14: F, t6056: F, t27519: F, t7203: F, t3789: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t150304 = t141369 * t35315;
    let t150308 = t140762 * t140833 * t140757 * t27845;
    let t150319 = sigma2 * t6817;
    let t150320 = t150319 * t14;
    let t150321 = t150320 * t6056;
    let t150327 = t27519 * t7203;
    let t150328 = t3789 * t150327;
    (t150304, t150308, t150319, t150321, t150327, t150328)
}
