//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 373/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk373(t213: f64, t218: f64, t215: f64, t608: f64, t211: f64, t408: f64, t90: f64, t220: f64, t612: f64, t43: f64, t1228: f64, t286: f64, t708: f64, t284: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t1653 = t215 * t215;
    let t1654 = 1.0_f64 / t1653;
    let t1655 = t608 * t608;
    let t1658 = t211 * t408;
    let t1660 = -2.0_f64 * t90 + 2.0_f64 * t1658;
    let t1664 = piecewise3(t214, 0.0_f64, 4.0_f64 / 9.0_f64 * t1654 * t1655 + 4.0_f64 / 3.0_f64 * t215 * t1660);
    let t1665 = t220 * t220;
    let t1666 = 1.0_f64 / t1665;
    let t1667 = t612 * t612;
    let t1670 = -t1660;
    let t1674 = piecewise3(t219, 0.0_f64, 4.0_f64 / 9.0_f64 * t1666 * t1667 + 4.0_f64 / 3.0_f64 * t220 * t1670);
    let t1676 = (t1664 + t1674) * t43;
    let t1681 = t1228 * t286 * t708;
    let t1683 = t284 * t284;
    (t1676, t1681, t1683)
}
