//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 527/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk527<F: Float>(t2074: F, t352: F, t262: F, t7192: F, t22: F, t880: F, t507: F) -> (F, F, F, F, F) {
    let t7193 = t2074 * t352;
    let t7194 = t262 * t7193;
    let t7195 = t7192 * t7194;
    let t7196 = F::new(0.27274661654245341728e-1) * t7195;
    let t7197 = t880 * t22;
    let t7198 = t507 * t7197;
    (t7193, t7194, t7196, t7197, t7198)
}
