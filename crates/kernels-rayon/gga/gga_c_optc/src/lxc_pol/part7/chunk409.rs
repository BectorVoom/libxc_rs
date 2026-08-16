//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 409/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk409(t108: f64, t2002: f64, t56: f64, t117: f64, t623: f64, t627: f64, t631: f64, t138: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2003 = t108 * t2002;
    let t2004 = t2003 * t56;
    let t2006 = 35.0_f64 / 432.0_f64 * t2004 * t117;
    let t2007 = t623 * t627;
    let t2008 = t2007 * t631;
    let t2010 = t56 * t138;
    (t2003, t2004, t2006, t2007, t2008, t2010)
}
