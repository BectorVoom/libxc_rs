//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2209/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2209(t2126: f64, t2371: f64, t13514: f64, t1519: f64, t2163: f64, t2322: f64, t2328: f64, t27060: f64, t29337: f64, t29432: f64, t29459: f64, t4257: f64, t4293: f64, t651: f64, t670: f64, t8233: f64, t97610: f64, t97617: f64, t97629: f64, t97639: f64, t97641: f64, t97643: f64, t97645: f64, t97647: f64, t97649: f64, t97653: f64, t97657: f64, t97659: f64) -> (f64, f64) {
    let t104138 = t2126 * t2371;
    let t104153 = -2.0_f64 * t13514 * t2163 * t651 - 4.0_f64 * t29337 * t651 * t670 - 2.0_f64 * t104138 * t1519 - 4.0_f64 * t2322 * t29459 - 2.0_f64 * t2328 * t8233 - 4.0_f64 * t27060 * t4293 - 4.0_f64 * t29432 * t4257 - t97610 - t97617 - t97629 - t97639 - t97641 - t97643 - t97645 - t97647 - t97649 + t97653 + t97657 + t97659;
    (t104138, t104153)
}
