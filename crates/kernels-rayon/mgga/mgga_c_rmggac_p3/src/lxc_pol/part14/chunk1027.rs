//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1027/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1027(t118: f64, t25877: f64, t326: f64, t338: f64, t352: f64, t40940: f64, t41129: f64, t41386: f64, t41393: f64, t41395: f64, t41402: f64, t41405: f64, t41409: f64, t41412: f64, t41414: f64, t5155: f64, t5266: f64, t839: f64, t848: f64, t8946: f64, t8975: f64) -> f64 {
    let t41420 = -t41129 + 0.19957069503106347607e-1_f64 * t118 * t338 * t41386 + 0.23948483403727617128e0_f64 * t5266 * t40940 * t352 + 0.44903406381989282115e-1_f64 * t41393 + 0.35922725105591425692e0_f64 * t41395 + 0.71845450211182851384e0_f64 * t25877 * t8975 * t839 + 0.13637330827122670864e0_f64 * t41402 + 0.16364796992547205037e0_f64 * t41405 + 0.40911992481368012592e-1_f64 * t41409 - 0.81823984962736025184e-1_f64 * t41412 - 0.59871208509319042821e-1_f64 * t326 * t41414 + 0.23948483403727617128e0_f64 * t5155 * t8946 * t848;
    t41420
}
