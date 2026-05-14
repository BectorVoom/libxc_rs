//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 895/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk895<F: Float>(t21289: F, t2257: F, t3783: F, t469: F, t6387: F, t4229: F, t5885: F, t13329: F, t2339: F, t4534: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t21290 = t21289 * sigma0;
    let t21314 = t2257 * t3783;
    let t21321 = t6387 * t469;
    let t21331 = t5885 * t4229;
    let t21334 = t13329 * t4229;
    let t21345 = t2339 * t4534;
    (t21290, t21314, t21321, t21331, t21334, t21345)
}
