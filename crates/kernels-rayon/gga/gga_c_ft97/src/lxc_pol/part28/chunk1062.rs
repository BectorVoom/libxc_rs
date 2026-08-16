//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1062/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1062(t137231: f64, t1564: f64, t446: f64, t920: f64, t32325: f64, t942: f64, t1317: f64, t1800: f64, t28: f64, t34401: f64, t376: f64, t145607: f64, t145611: f64, t145615: f64, t145619: f64, t145621: f64, t145626: f64, t145628: f64, t145632: f64, t145636: f64, t145640: f64, t145644: f64, t145648: f64, t145652: f64) -> (f64, f64, f64, f64, f64) {
    let t145656 = t446 * t1564 * t137231 * t920;
    let t145658 = t32325 * t942;
    let t145661 = t1317 * t28 * t1800 * t145658;
    let t145663 = t1317 * t376 * t34401;
    let t145665 = t145607 / 6.0_f64 - t145611 / 2.0_f64 - 3.0_f64 / 8.0_f64 * t145615 - 6.0_f64 * t145619 + t145621 / 9.0_f64 + t145626 / 6.0_f64 - t145628 / 9.0_f64 - t145632 / 2.0_f64 - 12.0_f64 * t145636 + 24.0_f64 * t145640 - 12.0_f64 * t145644 - 6.0_f64 * t145648 + 2.0_f64 / 3.0_f64 * t145652 + t145656 / 3.0_f64 + t145661 - 2.0_f64 / 3.0_f64 * t145663;
    (t145656, t145658, t145661, t145663, t145665)
}
