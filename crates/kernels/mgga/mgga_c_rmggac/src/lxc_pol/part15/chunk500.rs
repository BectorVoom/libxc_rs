//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 500/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk500<F: Float>(t638: F, t641: F, t7184: F, t338: F, t837: F, t22: F, t235: F) -> (F, F, F, F) {
    let t7186 = t638 * t7184 * t641;
    let t7190 = t837 * t338;
    let t7191 = t7190 * t22;
    let t7192 = t235 * t7191;
    (t7186, t7190, t7191, t7192)
}
