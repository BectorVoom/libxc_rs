//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1115/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1115(t4905: f64, t9523: f64, t118: f64, t25854: f64, t25877: f64, t333: f64, t352: f64, t36058: f64, t41488: f64, t41490: f64, t41492: f64, t43377: f64, t43380: f64, t43655: f64, t44239: f64, t44244: f64, t4669: f64, t5148: f64, t5259: f64, t833: f64, t848: f64) -> (f64, f64) {
    let t44277 = t9523 * t4905;
    let t44292 = 0.11974241701863808564e0_f64 * t5259 * t9523 * t833 - 0.39914139006212695214e-1_f64 * t118 * t43655 + 0.14369090042236570277e1_f64 * t25877 * t43377 + 0.35922725105591425692e0_f64 * t41488 + 0.23948483403727617128e0_f64 * t41490 - 0.35922725105591425692e0_f64 * t41492 + 0.71845450211182851384e0_f64 * t25854 * t44277 - 0.35922725105591425692e0_f64 * t4669 * t44239 * t333 - 0.17961362552795712846e0_f64 * t4669 * t9523 * t848 + 0.71845450211182851384e0_f64 * t25854 * t43380 - 0.5854073720911195298e0_f64 * t36058 - 0.23948483403727617128e0_f64 * t5148 * t44244 * t352;
    (t44277, t44292)
}
