//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 512/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk512<F: Float>(t2046: F, t2051: F, t7297: F, t270: F, t303: F, t2039: F, t638: F, t357: F, t36: F, t4789: F, t71: F) -> (F, F, F, F, F, F) {
    let t7299 = t2046 * t7297 * t2051;
    let t7301 = t303 * t270;
    let t7303 = t638 * t2039 * t7301;
    let t7304 = 0.30487649791575028314e-3 * t7303;
    let t7305 = t357 * t270;
    let t7307 = t638 * t2039 * t7305;
    let t7308 = 0.30487649791575028314e-3 * t7307;
    let t7310 = t36 * t4789 * t71;
    (t7299, t7301, t7304, t7305, t7308, t7310)
}
