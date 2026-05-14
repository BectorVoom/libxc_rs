//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 497/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk497<F: Float>(t5384: F, t5410: F, t5442: F, t5749: F, t109: F, t574: F, t934: F, t352: F, t570: F) -> (F, F, F, F) {
    let t5751 = t5384 + t5410 + t5442 + t5749;
    let t5752 = t5751 * t109;
    let t5757 = t934 * t574;
    let t5888 = t570 * t352;
    (t5751, t5752, t5757, t5888)
}
