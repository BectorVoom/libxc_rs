//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 784/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk784<F: Float>(t7229: F, t7364: F, t275: F, t7885: F, t236: F, t3899: F, t507: F, t2186: F, t7677: F, t2004: F, t7921: F, t2007: F) -> (F, F, F, F, F, F) {
    let t36489 = t7229 * t7364;
    let t36499 = t275 * t7885;
    let t36504 = t507 * t236 * t3899;
    let t36505 = F::new(0.68297526743963945143e0) * t36504;
    let t36506 = t2186 * t7677;
    let t36508 = t7921 * t2004;
    let t36511 = t7921 * t2007;
    (t36489, t36499, t36505, t36506, t36508, t36511)
}
