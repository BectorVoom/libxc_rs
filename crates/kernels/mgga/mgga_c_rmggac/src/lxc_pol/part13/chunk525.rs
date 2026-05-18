//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 525/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk525<F: Float>(t638: F, t641: F, t7184: F, t4968: F, t681: F, t338: F, t837: F, t22: F, t235: F) -> (F, F, F, F, F) {
    let t7186 = t638 * t7184 * t641;
    let t7188 = t4968 * t681;
    let t7190 = t837 * t338;
    let t7191 = t7190 * t22;
    let t7192 = t235 * t7191;
    (t7186, t7188, t7190, t7191, t7192)
}
