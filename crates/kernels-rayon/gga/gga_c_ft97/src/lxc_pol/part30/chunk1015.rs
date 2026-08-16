//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1015/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1015(t141369: f64, t35315: f64, t140757: f64, t140762: f64, t140833: f64, t27845: f64, t6817: f64, t14: f64, t6056: f64, t27519: f64, t7203: f64, t3789: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t150304 = t141369 * t35315;
    let t150308 = t140762 * t140833 * t140757 * t27845;
    let t150319 = sigma2 * t6817;
    let t150320 = t150319 * t14;
    let t150321 = t150320 * t6056;
    let t150327 = t27519 * t7203;
    let t150328 = t3789 * t150327;
    (t150304, t150308, t150319, t150321, t150327, t150328)
}
