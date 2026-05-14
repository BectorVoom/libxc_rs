//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 587/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk587<F: Float>(t5272: F, t716: F, t736: F, t1871: F, t1929: F, t1937: F, t1931: F, t1941: F, t5060: F, t732: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t5273 = t5272 * t716;
    let t5274 = t5273 * sigma2;
    let t5275 = t5274 * t736;
    let t5277 = t1929 * t1871;
    let t5278 = t5277 * sigma2;
    let t5279 = t5278 * t1937;
    let t5281 = t1931 * t1941;
    let t5283 = t732 * t5060;
    (t5273, t5274, t5275, t5277, t5278, t5279, t5281, t5283)
}
