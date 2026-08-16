//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 908/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk908(t1160: f64, t2372: f64, t222: f64, t2382: f64, t226: f64, t1689: f64, t3771: f64, t6813: f64, t3722: f64, t2378: f64, t37481: f64, t223: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65408 = t2372 * t1160;
    let t65692 = t2382 * t222;
    let t65693 = t65692 * t226;
    let t66076 = t3771 * t6813 * t1689;
    let t66382 = t3722 * t222;
    let t66422 = t37481 * t2378;
    let t66563 = t3722 * t223;
    (t65408, t65692, t65693, t66076, t66382, t66422, t66563)
}
