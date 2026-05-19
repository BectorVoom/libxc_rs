//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1031/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1031<F: Float>(t40134: F, t5259: F, t118: F, t27055: F, t333: F, t352: F, t36045: F, t36248: F, t39427: F, t40983: F, t41484: F, t41488: F, t41490: F, t41492: F, t41500: F, t41501: F, t4669: F, t5148: F, t833: F, t8936: F) -> F {
    let t41506 = t5259 * t40134;
    let t41511 = -F::cast_from(0.79828278012425390428e-1_f64) * t118 * t41484 - F::cast_from(0.79828278012425390426e-1_f64) * t36045 + F::cast_from(0.17961362552795712846e0_f64) * t41488 + F::cast_from(0.11974241701863808564e0_f64) * t41490 - F::cast_from(0.17961362552795712846e0_f64) * t41492 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t40983 * t333 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t40983 * t352 - t41500 + F::cast_from(0.5987120850931904282e-1_f64) * t41501 - F::cast_from(0.11974241701863808564e0_f64) * t5148 * t8936 * t833 - F::cast_from(0.2993560425465952141e-1_f64) * t41506 + F::cast_from(0.39914139006212695213e-1_f64) * t36248 - F::cast_from(0.71845450211182851384e0_f64) * t27055 * t39427;
    t41511
}
