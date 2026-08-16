//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 357/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk357(t1411: f64, t1480: f64, t1571: f64, t1607: f64, t572: f64, t575: f64, t208: f64, t574: f64) -> (f64, f64, f64) {
    let t1609 = t1411 + t1480 + t1571 + t1607;
    let t1611 = t572 * t575;
    let t1615 = 1.0_f64 / t574 / t208;
    (t1609, t1611, t1615)
}
