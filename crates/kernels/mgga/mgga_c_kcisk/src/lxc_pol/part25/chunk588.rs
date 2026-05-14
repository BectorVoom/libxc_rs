//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 588/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk588<F: Float>(t5283: F, t5063: F, t719: F, t735: F, t1934: F, t718: F, sigma2: F) -> (F, F, F, F, F) {
    let t5284 = t5283 * sigma2;
    let t5285 = t719 * t5063;
    let t5286 = t735 * t5285;
    let t5287 = t5284 * t5286;
    let t5289 = t1934 * t718;
    (t5284, t5285, t5286, t5287, t5289)
}
