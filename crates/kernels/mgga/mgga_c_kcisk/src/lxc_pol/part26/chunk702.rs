//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 702/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk702<F: Float>(t2271: F, t2279: F, t499: F, t8072: F, t498: F, t4235: F, t4231: F, t8077: F) -> (F, F, F, F, F) {
    let t8265 = t2271 * t2279;
    let t8267 = t499 * t8072;
    let t8268 = t498 * t8267;
    let t8269 = t4235 * t8268;
    let t8271 = t4231 * t8077;
    (t8265, t8267, t8268, t8269, t8271)
}
