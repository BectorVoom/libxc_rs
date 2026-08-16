//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 224/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk224(t113: f64, t122: f64, t60: f64, t684: f64, t718: f64, t728: f64, t745: f64, t97: f64) -> f64 {
    let t747 = -0.11713266981940447749e-2_f64 * t113 * t97 - 0.23426533963880895498e-2_f64 * t718 * t728 - t684 * t122 - t60 * t745;
    t747
}
