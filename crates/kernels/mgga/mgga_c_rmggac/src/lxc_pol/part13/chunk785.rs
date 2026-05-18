//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 785/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk785<F: Float>(t36271: F, t7204: F, t36277: F, t7192: F, t7244: F, t7484: F, t35383: F, t7473: F, t7450: F, t34884: F, t7751: F, t507: F, t7191: F) -> (F, F, F, F, F, F, F) {
    let t36416 = t7204 * t36271;
    let t36418 = t7192 * t36277;
    let t36448 = t7244 * t7484;
    let t36450 = t35383 * t7473;
    let t36453 = t7244 * t7450;
    let t36464 = t34884 * t7751;
    let t36471 = t507 * t7191;
    (t36416, t36418, t36448, t36450, t36453, t36464, t36471)
}
