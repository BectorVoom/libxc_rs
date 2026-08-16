//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 280/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk280(t252: f64, t798: f64, t154: f64, t782: f64, t222: f64, t119: f64, t776: f64, t210: f64) -> (f64, f64, f64, f64) {
    let t799 = t798 * t252;
    let t801 = t782 * t154;
    let t803 = 7.0_f64 / 288.0_f64 * t801 * t222;
    let t804 = t119 * t776;
    let t805 = t210 * t804;
    (t799, t801, t803, t805)
}
