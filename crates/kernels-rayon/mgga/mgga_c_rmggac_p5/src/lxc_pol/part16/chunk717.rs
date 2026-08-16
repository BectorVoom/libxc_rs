//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 717/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk717(t8129: f64, t8143: f64, t8156: f64, t8737: f64, t8739: f64, t8741: f64, t9904: f64, t9906: f64, t9909: f64, t9911: f64, t9913: f64, t9915: f64, t9917: f64, t9919: f64, t9921: f64, t9923: f64) -> f64 {
    let t10457 = -0.42483693136193860285e-2_f64 * t8737 - 0.15965655602485078085e0_f64 * t8739 + 0.10643770401656718724e0_f64 * t8741 + t8129 - 0.5454932330849068346e-1_f64 * t9904 - 0.25401708187682578962e-2_f64 * t9906 - t8143 - 0.19957069503106347607e-1_f64 * t9909 + 0.79656924630363488034e-3_f64 * t9911 - 0.66380770525302906695e-3_f64 * t9913 - 0.19957069503106347607e-1_f64 * t9915 + 0.2993560425465952141e-1_f64 * t9917 - 0.55759847241254441624e-2_f64 * t9919 - 0.11974241701863808564e0_f64 * t9921 - 0.26552308210121162678e-2_f64 * t9923 - t8156;
    t10457
}
