//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 885/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk885(t1017: f64, t1389: f64, t5778: f64, t28: f64, t32709: f64, t6587: f64, t32717: f64, t9073: f64, t925: f64, t1058: f64, t7313: f64, t1384: f64, t6718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35010 = t1389 * t1017;
    let t35011 = t5778 * t35010;
    let t35012 = t28 * t35011;
    let t35015 = t32709 * t6587;
    let t35016 = t28 * t35015;
    let t35022 = t9073 * t32717 * t925;
    let t35027 = t7313 * t1058;
    let t35028 = t28 * t35027;
    let t35033 = t1384 * t6718;
    (t35010, t35011, t35012, t35015, t35016, t35022, t35027, t35028, t35033)
}
