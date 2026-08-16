//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1031/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1031(t40134: f64, t5259: f64, t118: f64, t27055: f64, t333: f64, t352: f64, t36045: f64, t36248: f64, t39427: f64, t40983: f64, t41484: f64, t41488: f64, t41490: f64, t41492: f64, t41500: f64, t41501: f64, t4669: f64, t5148: f64, t833: f64, t8936: f64) -> f64 {
    let t41506 = t5259 * t40134;
    let t41511 = -0.79828278012425390428e-1_f64 * t118 * t41484 - 0.79828278012425390426e-1_f64 * t36045 + 0.17961362552795712846e0_f64 * t41488 + 0.11974241701863808564e0_f64 * t41490 - 0.17961362552795712846e0_f64 * t41492 - 0.35922725105591425692e0_f64 * t4669 * t40983 * t333 - 0.23948483403727617128e0_f64 * t5148 * t40983 * t352 - t41500 + 0.5987120850931904282e-1_f64 * t41501 - 0.11974241701863808564e0_f64 * t5148 * t8936 * t833 - 0.2993560425465952141e-1_f64 * t41506 + 0.39914139006212695213e-1_f64 * t36248 - 0.71845450211182851384e0_f64 * t27055 * t39427;
    t41511
}
