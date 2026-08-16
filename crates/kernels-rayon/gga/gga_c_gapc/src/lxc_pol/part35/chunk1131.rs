//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1131/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1131(t11761: f64, t34005: f64, t3775: f64, t9586: f64, t11428: f64, t667: f64, t3326: f64, t29576: f64, t29582: f64, t30153: f64, t30158: f64, t28427: f64, t3784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34036 = t34005 * t11761;
    let t34038 = t3775 * t9586;
    let t34040 = t667 * t11428;
    let t34041 = t34040 * t3326;
    let t34043 = t29576 * t34041 * t29582;
    let t34046 = t30153 * t34041 * t30158;
    let t34048 = t3784 * t28427;
    (t34036, t34038, t34040, t34043, t34046, t34048)
}
