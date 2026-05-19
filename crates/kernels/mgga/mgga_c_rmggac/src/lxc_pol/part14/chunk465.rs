//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 465/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk465<F: Float>(t352: F, t5126: F, t1615: F, t321: F, t118: F, t305: F, t326: F, t3814: F, t4974: F, t4977: F, t4982: F, t5005: F, t5008: F, t5029: F, t5033: F, t5052: F, t5064: F, t5072: F, t5076: F, t5095: F, t5099: F, t5103: F, t5108: F, t5116: F, t5121: F, t793: F, t797: F, t838: F) -> F {
    let t5127 = t5126 * t352;
    let t5130 = t1615 * t321;
    let t5133 = -F::cast_from(0.11974241701863808564e0_f64) * t793 * t5008 - F::cast_from(0.11974241701863808564e0_f64) * t305 * t5033 + F::cast_from(0.17961362552795712846e0_f64) * t797 * t5005 - F::cast_from(0.23948483403727617128e0_f64) * t793 * t5072 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t4977 + F::cast_from(0.35922725105591425692e0_f64) * t797 * t5076 - F::cast_from(0.23948483403727617128e0_f64) * t838 * t5095 + F::cast_from(0.11974241701863808564e0_f64) * t305 * t5099 + F::cast_from(0.11974241701863808564e0_f64) * t118 * t5103 + F::cast_from(0.11974241701863808564e0_f64) * t793 * t5052 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t5108 - F::cast_from(0.71845450211182851384e0_f64) * t3814 * t5064 - F::cast_from(0.11974241701863808564e0_f64) * t326 * t4974 - F::cast_from(0.79828278012425390428e-1_f64) * t118 * t5116 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t5029 - F::cast_from(0.47896966807455234256e0_f64) * t838 * t5121 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t4982 - F::cast_from(0.11974241701863808564e0_f64) * t326 * t5127 + F::cast_from(0.35922725105591425692e0_f64) * t797 * t5130;
    t5133
}
