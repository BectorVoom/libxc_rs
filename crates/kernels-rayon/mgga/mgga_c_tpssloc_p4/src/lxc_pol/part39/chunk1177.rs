//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1177/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1177(t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11247: f64, t14702: f64, t14708: f64, t14721: f64, t14723: f64, t14724: f64, t14728: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64) -> f64 {
    let t14758 = -t11247 + 8.0_f64 / 27.0_f64 * t11137 + 2.0_f64 / 27.0_f64 * t11139 - 2.0_f64 / 9.0_f64 * t11141 - t11143 / 9.0_f64 + 4.0_f64 / 27.0_f64 * t14702 + t14721 - t14723 - t14724 + 10.0_f64 / 27.0_f64 * t14728 - 4.0_f64 / 3.0_f64 * t14733 - 4.0_f64 / 9.0_f64 * t14738 - 2.0_f64 / 9.0_f64 * t14742 + 2.0_f64 * t14746 + 4.0_f64 / 3.0_f64 * t14751 + 2.0_f64 / 3.0_f64 * t14755 + t14708 / 3.0_f64;
    t14758
}
