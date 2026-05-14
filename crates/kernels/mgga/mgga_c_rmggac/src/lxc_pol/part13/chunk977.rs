//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 977/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk977<F: Float>(t4905: F, t9523: F, t118: F, t25854: F, t25877: F, t333: F, t352: F, t36058: F, t41488: F, t41490: F, t41492: F, t43377: F, t43380: F, t43655: F, t44239: F, t44244: F, t4669: F, t5148: F, t5259: F, t833: F, t848: F) -> (F, F) {
    let t44277 = t9523 * t4905;
    let t44292 = 0.11974241701863808564e0 * t5259 * t9523 * t833 - 0.39914139006212695214e-1 * t118 * t43655 + 0.14369090042236570277e1 * t25877 * t43377 + 0.35922725105591425692e0 * t41488 + 0.23948483403727617128e0 * t41490 - 0.35922725105591425692e0 * t41492 + 0.71845450211182851384e0 * t25854 * t44277 - 0.35922725105591425692e0 * t4669 * t44239 * t333 - 0.17961362552795712846e0 * t4669 * t9523 * t848 + 0.71845450211182851384e0 * t25854 * t43380 - 0.5854073720911195298e0 * t36058 - 0.23948483403727617128e0 * t5148 * t44244 * t352;
    (t44277, t44292)
}
