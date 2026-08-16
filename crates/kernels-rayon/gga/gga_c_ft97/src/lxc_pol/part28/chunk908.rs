//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 908/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk908(t1587: f64, t480: f64, t370: f64, t8216: f64, t971: f64, t1780: f64, t1852: f64, t1786: f64, t3238: f64, t463: f64, t8418: f64, t10: f64, t16: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47007 = t1587 * t480;
    let t47120 = t370 * t480;
    let t47273 = t8216 * t971;
    let t47399 = t1780 * t1852;
    let t47443 = t1786 * t3238;
    let t47548 = t463 * t8418;
    let t47659 = t10 * t16 * t378;
    (t47007, t47120, t47273, t47399, t47443, t47548, t47659)
}
