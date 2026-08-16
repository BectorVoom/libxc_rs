//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 368/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk368(t1628: f64, t601: f64, t570: f64, t575: f64, t1589: f64, t494: f64, t561: f64, t566: f64) -> (f64, f64, f64, f64, f64) {
    let t1629 = t1628 * t601;
    let t1632 = t1628 * t570;
    let t1635 = t1628 * t575;
    let t1638 = t1589 * t494;
    let t1641 = t561 * t566;
    (t1629, t1632, t1635, t1638, t1641)
}
