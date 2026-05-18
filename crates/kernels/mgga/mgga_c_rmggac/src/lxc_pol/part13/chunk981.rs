//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 981/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk981<F: Float>(t39372: F, t5148: F, t40864: F, t4669: F, t118: F, t2001: F, t352: F, t38523: F, t7720: F, t34884: F, t9118: F, t2283: F, t34881: F) -> (F, F, F, F, F) {
    let t41560 = t5148 * t39372;
    let t41562 = t4669 * t40864;
    let t41576 = t2001 * t118 * t38523 * t352;
    let t41577 = t7720 * t41576;
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    (t41560, t41562, t41577, t41579, t41581)
}
