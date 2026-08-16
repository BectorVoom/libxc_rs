//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3629/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3629(t16784: f64, t5198: f64, t12571: f64, t6548: f64, t1149: f64, t56265: f64, t57795: f64, t1196: f64, t17151: f64, t5197: f64, t16639: f64, t5192: f64) -> (f64, f64, f64, f64, f64) {
    let t68725 = 0.46785788981077169656e1_f64 * t16784 * t5198;
    let t68727 = 0.11696447245269292414e1_f64 * t12571 * t6548;
    let t68730 = 0.2069040516770936012e4_f64 * t57795 * t56265 * t1149;
    let t68733 = 0.23392894490538584828e1_f64 * t1196 * t5197 * t17151;
    let t68735 = 0.70178683471615754484e1_f64 * t5192 * t16639;
    (t68725, t68727, t68730, t68733, t68735)
}
