//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1764/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1764(t24744: f64, t24746: f64, t3523: f64, t7345: f64, t3572: f64, t7339: f64, t24574: f64, t7368: f64, t2148: f64, t3427: f64, t2121: f64, t225: f64, t24594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24747 = t24744 * t24746;
    let t24752 = t7345 * t3523;
    let t24754 = t7339 * t3572;
    let t24760 = t24574 * t7368;
    let t24771 = t3427 * t2148;
    let t24773 = 0.18277045187202515961e-2_f64 * t2121 * t24771;
    let t24776 = t24594 * t225;
    (t24747, t24752, t24754, t24760, t24771, t24773, t24776)
}
