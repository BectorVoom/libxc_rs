//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1217/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1217(t39762: f64, t37859: f64, t37881: f64, t39759: f64, t39765: f64, t39768: f64, t39770: f64, t39772: f64, t39775: f64, t39778: f64, t39780: f64, t39782: f64) -> f64 {
    let t41542 = 0.13869154784086829701e1_f64 * t39762;
    let t41551 = 0.46230515946956099004e0_f64 * t37859 + 0.95219938395347901946e-2_f64 * t37881 - 0.10401866088065122276e1_f64 * t39759 - t41542 - 0.5200933044032561138e0_f64 * t39765 - 0.52009330440325611378e0_f64 * t39768 + 0.51220160311720645766e0_f64 * t39770 - 0.85366933852867742943e0_f64 * t39772 - 0.17465477326173296718e-1_f64 * t39775 - 0.26198215989259945076e-1_f64 * t39778 - 0.26198215989259945076e-1_f64 * t39780 - 0.1047928639570397803e0_f64 * t39782;
    t41551
}
