//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 904/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk904<F: Float>(t874: F, t8794: F, t352: F, t25820: F, t38977: F, t27101: F, t38980: F, t25854: F, t38983: F, t36058: F, t6444: F, t9005: F, t40134: F, t5259: F, t118: F, t27055: F, t333: F, t36045: F, t36248: F, t39427: F, t40983: F, t4669: F, t5148: F, t833: F, t8936: F) -> (F, F) {
    let t41483 = t874 * t8794;
    let t41484 = t41483 * t352;
    let t41488 = t25820 * t38977;
    let t41490 = t27101 * t38980;
    let t41492 = t25854 * t38983;
    let t41500 = 0.2927036860455597649e0 * t36058;
    let t41501 = t6444 * t9005;
    let t41506 = t5259 * t40134;
    let t41511 = -0.79828278012425390428e-1 * t118 * t41484 - 0.79828278012425390426e-1 * t36045 + 0.17961362552795712846e0 * t41488 + 0.11974241701863808564e0 * t41490 - 0.17961362552795712846e0 * t41492 - 0.35922725105591425692e0 * t4669 * t40983 * t333 - 0.23948483403727617128e0 * t5148 * t40983 * t352 - t41500 + 0.5987120850931904282e-1 * t41501 - 0.11974241701863808564e0 * t5148 * t8936 * t833 - 0.2993560425465952141e-1 * t41506 + 0.39914139006212695213e-1 * t36248 - 0.71845450211182851384e0 * t27055 * t39427;
    (t41484, t41511)
}
