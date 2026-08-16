//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 691/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk691(t20123: f64, t20161: f64, t348: f64, t4572: f64, t925: f64, t8557: f64, t4436: f64, t979: f64, t1871: f64, t488: f64, t4495: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20162 = t20123 + t20161;
    let t20163 = t348 * t20162;
    let t20171 = t4572 * t925;
    let t20172 = t8557 * t20171;
    let t20177 = t4436 * t979;
    let t20179 = t1871 * t488 * t20177;
    let t20182 = t942 * t4495;
    (t20162, t20163, t20171, t20172, t20177, t20179, t20182)
}
