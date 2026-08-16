//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 247/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk247(t207: f64, t792: f64, t795: f64, t785: f64, t787: f64, t789: f64, t252: f64, t154: f64, t782: f64, t222: f64, t119: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t797 = 0.41666666666666666666e-3_f64 * t792 * t207 * t795;
    let t798 = -t785 - 0.16666666666666666666e-2_f64 * t787 * t789 - t797;
    let t799 = t798 * t252;
    let t801 = t782 * t154;
    let t803 = 7.0_f64 / 288.0_f64 * t801 * t222;
    let t804 = t119 * t776;
    (t797, t798, t799, t801, t803, t804)
}
