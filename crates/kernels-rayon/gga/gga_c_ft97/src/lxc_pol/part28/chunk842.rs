//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 842/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk842(t34482: f64, t469: f64, t1317: f64, t28: f64, t32333: f64, t7824: f64, t920: f64, t446: f64, t32338: f64, t942: f64, t89: f64, t5507: f64, t6454: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34483 = t469 * t34482;
    let t34485 = t1317 * t28 * t34483;
    let t34488 = t7824 * t32333 * t920;
    let t34489 = t446 * t34488;
    let t34491 = t32338 * t942;
    let t34492 = t28 * t34491;
    let t34493 = t89 * t34492;
    let t34495 = t5507 * t6454;
    (t34483, t34485, t34488, t34489, t34491, t34493, t34495)
}
