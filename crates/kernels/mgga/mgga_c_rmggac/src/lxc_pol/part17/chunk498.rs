//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 498/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk498<F: Float>(t352: F, t6608: F, t118: F, t305: F, t326: F, t3851: F, t4669: F, t5155: F, t5919: F, t5922: F, t5925: F, t5937: F, t5949: F, t5966: F, t6327: F, t6363: F, t6495: F, t6501: F, t6504: F, t6508: F, t6570: F, t6583: F, t6586: F, t6590: F, t6592: F, t6599: F, t6602: F, t793: F, t797: F, t838: F) -> (F,) {
    let t6609 = t6608 * t352;
    let t6616 = 0.11974241701863808564e0 * t326 * t5966 - 0.23948483403727617128e0 * t793 * t6570 - 0.79828278012425390428e-1 * t118 * t5937 - 0.47896966807455234256e0 * t838 * t6327 - 0.11974241701863808564e0 * t793 * t6501 + 0.17961362552795712846e0 * t797 * t6504 + 0.23948483403727617128e0 * t838 * t5949 - 0.35922725105591425692e0 * t4669 * t6583 + 0.47896966807455234256e0 * t5155 * t6586 - 0.59871208509319042821e-1 * t6590 - 0.11974241701863808564e0 * t6592 - 0.39914139006212695214e-1 * t118 * t5919 + 0.59871208509319042821e-1 * t305 * t5925 + 0.59871208509319042821e-1 * t6599 + 0.19957069503106347607e-1 * t6602 - 0.59871208509319042821e-1 * t326 * t5922 + 0.35922725105591425692e0 * t3851 * t6495 + 0.11974241701863808564e0 * t793 * t6609 + 0.11974241701863808564e0 * t118 * t6363 + 0.59871208509319042821e-1 * t305 * t6508;
    (t6616,)
}
