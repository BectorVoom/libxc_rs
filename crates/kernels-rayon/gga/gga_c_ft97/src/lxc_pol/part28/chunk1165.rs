//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1165/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1165(t139453: f64, t139485: f64, t140041: f64, t140042: f64, t148593: f64, t148597: f64, t148601: f64, t148604: f64, t148607: f64, t148611: f64, t148616: f64, t148621: f64, t148625: f64, t148629: f64, t148632: f64, t148636: f64) -> f64 {
    let t148844 = 2.0_f64 * t148593 + 4.0_f64 * t148597 - 6.0_f64 * t148601 - 2.0_f64 / 3.0_f64 * t148604 - 4.0_f64 / 3.0_f64 * t148607 + t139453 / 3.0_f64 + 2.0_f64 * t148611 + t148616 + t148621 / 4.0_f64 - 12.0_f64 * t148625 + t139485 / 9.0_f64 + t148629 + t148632 / 3.0_f64 - t148636 - t140041 + t140042;
    t148844
}
