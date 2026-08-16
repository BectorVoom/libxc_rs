//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 949/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk949(t11983: f64, t16640: f64, t16658: f64, t22353: f64, t22355: f64, t28231: f64, t28235: f64, t28239: f64, t28244: f64, t28250: f64, t28253: f64, t28259: f64) -> f64 {
    let t29782 = -0.46429444444444444443e-2_f64 * t28231 - 0.46429444444444444443e-2_f64 * t28235 - 0.58036805555555555555e-2_f64 * t28239 + 0.38691203703703703703e-2_f64 * t28244 + 0.38691203703703703703e-2_f64 * t22353 + 0.23214722222222222222e-2_f64 * t22355 + 0.69644166666666666665e-2_f64 * t28250 + 0.58036805555555555555e-2_f64 * t28253 + t11983 - 0.11607361111111111111e-2_f64 * t16640 + 0.69644166666666666665e-2_f64 * t28259 - 0.77382407407407407405e-3_f64 * t16658;
    t29782
}
