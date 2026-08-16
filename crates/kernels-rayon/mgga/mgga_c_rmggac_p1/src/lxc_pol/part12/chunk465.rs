//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 465/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk465(t352: f64, t5126: f64, t1615: f64, t321: f64, t118: f64, t305: f64, t326: f64, t3814: f64, t4974: f64, t4977: f64, t4982: f64, t5005: f64, t5008: f64, t5029: f64, t5033: f64, t5052: f64, t5064: f64, t5072: f64, t5076: f64, t5095: f64, t5099: f64, t5103: f64, t5108: f64, t5116: f64, t5121: f64, t793: f64, t797: f64, t838: f64) -> f64 {
    let t5127 = t5126 * t352;
    let t5130 = t1615 * t321;
    let t5133 = -0.11974241701863808564e0_f64 * t793 * t5008 - 0.11974241701863808564e0_f64 * t305 * t5033 + 0.17961362552795712846e0_f64 * t797 * t5005 - 0.23948483403727617128e0_f64 * t793 * t5072 + 0.59871208509319042821e-1_f64 * t305 * t4977 + 0.35922725105591425692e0_f64 * t797 * t5076 - 0.23948483403727617128e0_f64 * t838 * t5095 + 0.11974241701863808564e0_f64 * t305 * t5099 + 0.11974241701863808564e0_f64 * t118 * t5103 + 0.11974241701863808564e0_f64 * t793 * t5052 - 0.39914139006212695214e-1_f64 * t118 * t5108 - 0.71845450211182851384e0_f64 * t3814 * t5064 - 0.11974241701863808564e0_f64 * t326 * t4974 - 0.79828278012425390428e-1_f64 * t118 * t5116 - 0.59871208509319042821e-1_f64 * t326 * t5029 - 0.47896966807455234256e0_f64 * t838 * t5121 + 0.59871208509319042821e-1_f64 * t305 * t4982 - 0.11974241701863808564e0_f64 * t326 * t5127 + 0.35922725105591425692e0_f64 * t797 * t5130;
    t5133
}
