//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 627/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk627(t352: f64, t8264: f64, t2228: f64, t321: f64, t699: f64, t848: f64, t333: f64, t118: f64, t305: f64, t326: f64, t4669: f64, t7845: f64, t7847: f64, t7849: f64, t7853: f64, t7856: f64, t7863: f64, t7865: f64, t7867: f64, t7869: f64, t7877: f64, t793: f64, t8042: f64, t8045: f64, t8063: f64, t8078: f64, t8258: f64, t8261: f64, t838: f64) -> (f64, f64, f64, f64, f64) {
    let t8265 = t8264 * t352;
    let t8273 = t2228 * t321;
    let t8278 = t699 * t848;
    let t8281 = t2228 * t333;
    let t8290 = -0.40911992481368012596e-1_f64 * t7845 - 0.20455996240684006298e-1_f64 * t7847 + 0.2727466165424534173e-1_f64 * t7849 + 0.68186654135613354325e-2_f64 * t7853 - 0.2993560425465952141e-1_f64 * t7856 - 0.39914139006212695214e-1_f64 * t118 * t8258 - 0.35922725105591425692e0_f64 * t4669 * t8261 - 0.79828278012425390428e-1_f64 * t118 * t8265 + 0.11974241701863808564e0_f64 * t793 * t8063 + 0.17961362552795712846e0_f64 * t7863 - 0.35922725105591425692e0_f64 * t7865 - 0.11974241701863808564e0_f64 * t7867 + 0.11974241701863808564e0_f64 * t305 * t8273 + 0.59871208509319042821e-1_f64 * t305 * t8078 - 0.59871208509319042821e-1_f64 * t326 * t8278 - 0.11974241701863808564e0_f64 * t326 * t8281 + 0.23948483403727617128e0_f64 * t838 * t8045 + 0.11974241701863808564e0_f64 * t118 * t8042 + 0.35922725105591425692e0_f64 * t7869 + 0.5987120850931904282e-1_f64 * t7877;
    (t8265, t8273, t8278, t8281, t8290)
}
