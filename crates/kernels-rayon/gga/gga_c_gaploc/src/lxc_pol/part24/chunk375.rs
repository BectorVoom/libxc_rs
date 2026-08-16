//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 375/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk375(t215: f64, t220: f64, t1228: f64, t286: f64, t708: f64, t284: f64, t712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1653 = t215 * t215;
    let t1654 = 1.0_f64 / t1653;
    let t1665 = t220 * t220;
    let t1666 = 1.0_f64 / t1665;
    let t1681 = t1228 * t286 * t708;
    let t1683 = t284 * t284;
    let t1685 = 1.0_f64 / t1683 / t284;
    let t1687 = t1685 * pi * t712;
    (t1654, t1666, t1681, t1683, t1685, t1687)
}
