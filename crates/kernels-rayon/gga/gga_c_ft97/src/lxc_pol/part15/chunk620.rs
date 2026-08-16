//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 620/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk620(t2345: f64, t26: f64, t2347: f64, t743: f64, t666: f64, t2360: f64, t1131: f64, t2506: f64, t1087: f64, t89: f64, t9733: f64, t1132: f64, t1636: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13682 = t26 * t2345;
    let t13683 = t743 * t2347;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13693 = t2506 * t1131;
    let t13722 = t89 * t9733 * t1087;
    let t13739 = t89 * t1636 * t1132;
    (t13682, t13683, t13688, t13689, t13693, t13722, t13739)
}
