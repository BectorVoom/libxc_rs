//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 526/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk526<F: Float>(t638: F, t641: F, t7184: F, t4968: F, t681: F, t338: F, t837: F, t22: F, t235: F) -> (F, F, F, F, F) {
    let t7186 = t638 * t7184 * t641;
    let t7188 = t4968 * t681;
    let t7189 = F::cast_from(0.2993560425465952141e-1_f64) * t7188;
    let t7190 = t837 * t338;
    let t7191 = t7190 * t22;
    let t7192 = t235 * t7191;
    (t7186, t7189, t7190, t7191, t7192)
}
