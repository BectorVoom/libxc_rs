//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 841/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk841<F: Float>(t41548: F, t793: F, t1347: F, t2408: F, t118: F, t2001: F, t352: F, t38523: F, t34884: F, t9118: F, t2283: F, t34881: F) -> (F, F, F, F, F) {
    let t41549 = t793 * t41548;
    let t41550 = F::cast_from(0.15965655602485078085e0_f64) * t41549;
    let t41571 = t1347 * t2408;
    let t41576 = t2001 * t118 * t38523 * t352;
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    (t41550, t41571, t41576, t41579, t41581)
}
