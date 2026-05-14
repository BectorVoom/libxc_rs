//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 232/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk232<F: Float>(t222: F, t390: F, t1056: F, t295: F, t298: F, t301: F, t430: F, sigma0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t1151 = 1.0 / t390;
    let t1152 = sigma0 * t1151;
    let t1155 = piecewise3(t223, 0.0, t1056);
    let t1156 = t295 * t1155;
    let t1161 = t298 * t430 * t301;
    (t1151, t1152, t1155, t1156, t1161)
}
