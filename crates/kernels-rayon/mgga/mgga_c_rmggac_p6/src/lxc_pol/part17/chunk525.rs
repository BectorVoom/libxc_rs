//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 525/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk525(t338: f64, t6463: f64, t118: f64, t1704: f64, t352: f64, t305: f64, t326: f64, t3851: f64, t4669: f64, t5155: f64, t5919: f64, t5922: f64, t5925: f64, t5937: f64, t5949: f64, t5966: f64, t6327: f64, t6363: f64, t6495: f64, t6501: f64, t6504: f64, t6508: f64, t6570: f64, t6583: f64, t6586: f64, t6590: f64, t6592: f64, t6599: f64, t793: f64, t797: f64, t838: f64) -> (f64, f64) {
    let t6601 = t338 * t6463;
    let t6602 = t118 * t6601;
    let t6608 = t338 * t1704;
    let t6609 = t6608 * t352;
    let t6616 = 0.11974241701863808564e0_f64 * t326 * t5966 - 0.23948483403727617128e0_f64 * t793 * t6570 - 0.79828278012425390428e-1_f64 * t118 * t5937 - 0.47896966807455234256e0_f64 * t838 * t6327 - 0.11974241701863808564e0_f64 * t793 * t6501 + 0.17961362552795712846e0_f64 * t797 * t6504 + 0.23948483403727617128e0_f64 * t838 * t5949 - 0.35922725105591425692e0_f64 * t4669 * t6583 + 0.47896966807455234256e0_f64 * t5155 * t6586 - 0.59871208509319042821e-1_f64 * t6590 - 0.11974241701863808564e0_f64 * t6592 - 0.39914139006212695214e-1_f64 * t118 * t5919 + 0.59871208509319042821e-1_f64 * t305 * t5925 + 0.59871208509319042821e-1_f64 * t6599 + 0.19957069503106347607e-1_f64 * t6602 - 0.59871208509319042821e-1_f64 * t326 * t5922 + 0.35922725105591425692e0_f64 * t3851 * t6495 + 0.11974241701863808564e0_f64 * t793 * t6609 + 0.11974241701863808564e0_f64 * t118 * t6363 + 0.59871208509319042821e-1_f64 * t305 * t6508;
    (t6602, t6616)
}
