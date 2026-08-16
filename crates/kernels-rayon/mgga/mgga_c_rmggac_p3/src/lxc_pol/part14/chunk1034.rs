//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1034/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1034(t41548: f64, t793: f64, t798: f64, t8975: f64, t4048: f64, t39372: f64, t5148: f64, t40864: f64, t4669: f64, t118: f64, t25820: f64, t27101: f64, t321: f64, t40589: f64, t41063: f64, t41535: f64, t41537: f64, t41538: f64, t41540: f64, t41542: f64, t41544: f64) -> (f64, f64, f64) {
    let t41549 = t793 * t41548;
    let t41550 = 0.15965655602485078085e0_f64 * t41549;
    let t41551 = t8975 * t798;
    let t41554 = t8975 * t4048;
    let t41560 = t5148 * t39372;
    let t41562 = t4669 * t40864;
    let t41564 = -t41535 - t41537 + 0.71845450211182851384e0_f64 * t41538 + 0.17961362552795712846e0_f64 * t41540 - 0.35922725105591425692e0_f64 * t41542 + 0.17961362552795712846e0_f64 * t41544 - 0.39914139006212695214e-1_f64 * t118 * t40589 - t41550 - 0.71845450211182851384e0_f64 * t25820 * t41551 - 0.47896966807455234256e0_f64 * t27101 * t41554 - 0.23948483403727617128e0_f64 * t5148 * t41063 * t321 + 0.2993560425465952141e-1_f64 * t41560 + 0.44903406381989282115e-1_f64 * t41562;
    (t41551, t41554, t41564)
}
