//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 586/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk586(t231: f64, t3817: f64, t3750: f64, t679: f64, t200: f64, t1095: f64, t3773: f64, t6027: f64, t1613: f64, t6789: f64, t6793: f64, t17836: f64, t6033: f64) -> (f64, f64, f64, f64, f64) {
    let t27588 = t231 * t3817;
    let t27595 = t679 * t3750;
    let t27596 = t27595 * t200;
    let t27601 = t3773 * t6027 * t1095;
    let t27604 = t1613 * t6789;
    let t27605 = t27604 * t6793;
    let t27609 = t17836 * t6033;
    (t27588, t27596, t27601, t27605, t27609)
}
