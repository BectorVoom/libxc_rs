//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1140/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1140(t10420: f64, t10481: f64, t2868: f64, t302: f64, t36748: f64, t36754: f64, t36756: f64, t38060: f64, t4041: f64, t47690: f64, t47694: f64, t47698: f64, t47702: f64, t47706: f64, t47710: f64, t47714: f64, t47719: f64, t5055: f64, t5928: f64, t72: f64, t9318: f64, t9321: f64, t9340: f64) -> f64 {
    let t49649 = 0.35922725105591425692e0_f64 * t5055 * t9318 + 0.23948483403727617128e0_f64 * t2868 * t9321 + t72 * t302 * t10481 - 0.30487649791575028312e-3_f64 * t36748 - t38060 - 0.30487649791575028312e-3_f64 * t36754 + 0.60975299583150056624e-3_f64 * t36756 + 0.1064114997332445985e-4_f64 * t47690 + 0.43368970657079495308e-4_f64 * t47694 - 0.30487649791575028312e-3_f64 * t47698 - 0.60975299583150056624e-3_f64 * t47702 + 0.86737941314158990616e-4_f64 * t47706 - 0.30487649791575028312e-3_f64 * t47710 + 0.43368970657079495308e-4_f64 * t47714 + 0.59871208509319042821e-1_f64 * t4041 * t10420 - 0.47896966807455234256e0_f64 * t47719 + 0.79828278012425390428e-1_f64 * t5928 * t9340;
    t49649
}
