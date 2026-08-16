//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 461/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk461(t352: f64, t9540: f64, t118: f64, t305: f64, t326: f64, t4669: f64, t5259: f64, t5266: f64, t7793: f64, t7796: f64, t7816: f64, t8919: f64, t8921: f64, t8926: f64, t9332: f64, t9340: f64, t9352: f64, t9370: f64, t9524: f64, t9527: f64, t9531: f64) -> (f64, f64) {
    let t9541 = t9540 * t352;
    let t9546 = -0.20455996240684006298e-1_f64 * t8919 + 0.2727466165424534173e-1_f64 * t8921 + 0.68186654135613354325e-2_f64 * t8926 + 0.79828278012425390427e-1_f64 * t7793 - 0.17961362552795712846e0_f64 * t4669 * t9524 + 0.11974241701863808564e0_f64 * t5259 * t9527 - 0.39914139006212695214e-1_f64 * t118 * t9531 + t7796 - 0.59871208509319042821e-1_f64 * t326 * t9352 - 0.39914139006212695214e-1_f64 * t118 * t9340 - 0.59871208509319042821e-1_f64 * t326 * t9370 + 0.11974241701863808564e0_f64 * t5266 * t9541 + 0.59871208509319042821e-1_f64 * t305 * t9332 - t7816;
    (t9541, t9546)
}
