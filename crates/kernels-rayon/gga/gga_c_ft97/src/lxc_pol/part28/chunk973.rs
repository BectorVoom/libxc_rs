//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 973/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk973(t1349: f64, t32875: f64, t376: f64, t1546: f64, t32664: f64, t1526: f64, t5917: f64, t7705: f64, t1774: f64, t5925: f64, t7298: f64, t32658: f64, t32661: f64) -> (f64, f64, f64, f64, f64) {
    let t138560 = t1349 * t376 * t32875;
    let t138568 = t1349 * t1546 * t32664;
    let t138586 = t1526 * t7705 * t5917;
    let t138598 = t7298 * t1774 * t5925;
    let t138607 = t32658 * t32661;
    (t138560, t138568, t138586, t138598, t138607)
}
