//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 784/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk784(t1775: f64, t2775: f64, t10589: f64, t10591: f64, t10594: f64, t10595: f64, t10597: f64, t10600: f64, t10604: f64, t10607: f64, t10611: f64, t10614: f64, t10617: f64, t462: f64, t92: f64) -> f64 {
    let t10619 = t1775 * t2775;
    let t10621 = t10589 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t10591 - t10594 - 4.0_f64 / 3.0_f64 * t10595 - 2.0_f64 * t462 * t10597 + 2.0_f64 * t462 * t10600 - 2.0_f64 * t462 * t10604 - 2.0_f64 * t462 * t10607 - t92 * t10611 + 2.0_f64 / 3.0_f64 * t462 * t10614 - 2.0_f64 / 3.0_f64 * t10617 - 2.0_f64 / 3.0_f64 * t10619;
    t10621
}
