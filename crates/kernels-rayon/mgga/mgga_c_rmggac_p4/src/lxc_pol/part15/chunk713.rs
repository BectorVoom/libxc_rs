//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 713/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk713(t10002: f64, t10120: f64, t10124: f64, t10130: f64, t10135: f64, t10137: f64, t10141: f64, t10143: f64, t10148: f64, t10151: f64, t118: f64, t305: f64, t326: f64, t4669: f64, t5148: f64, t7819: f64, t7821: f64, t793: f64, t8911: f64, t8913: f64, t8917: f64, t9840: f64, t9858: f64, t9867: f64, t9944: f64) -> f64 {
    let t10153 = 0.54549323308490683457e-1_f64 * t8911 - 0.72732431077987577943e-1_f64 * t8913 - 0.18183107769496894486e-1_f64 * t8917 + 0.13637330827122670864e-1_f64 * t10120 + 0.34093327067806677161e-2_f64 * t10124 + 0.11974241701863808564e0_f64 * t305 * t9858 + 0.11974241701863808564e0_f64 * t793 * t9840 - 0.23948483403727617128e0_f64 * t5148 * t10130 + 0.59871208509319042821e-1_f64 * t305 * t10002 - 0.17961362552795712846e0_f64 * t10135 - 0.5987120850931904282e-1_f64 * t10137 - 0.79828278012425390428e-1_f64 * t118 * t9867 + 0.17961362552795712846e0_f64 * t10141 - 0.35922725105591425692e0_f64 * t4669 * t10143 + t7819 - t7821 - 0.11974241701863808564e0_f64 * t326 * t9944 - 0.59871208509319042821e-1_f64 * t326 * t10148 + 0.2993560425465952141e-1_f64 * t10151;
    t10153
}
