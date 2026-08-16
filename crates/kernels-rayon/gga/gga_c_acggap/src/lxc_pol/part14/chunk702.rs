//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 702/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk702(t1111: f64, t604: f64, t1181: f64, t7426: f64, t2070: f64, t7433: f64, t2450: f64, t7336: f64) -> (f64, f64, f64, f64, f64) {
    let t7569 = t604 * t1111;
    let t7570 = t1181 * t7569;
    let t7571 = t7426 * t7570;
    let t7573 = t7433 * t2070;
    let t7575 = t2450 * t7336;
    (t7569, t7570, t7571, t7573, t7575)
}
