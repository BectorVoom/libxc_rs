//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 725/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk725<F: Float>(t271: F, t3899: F, t638: F, t641: F, t36293: F, t739: F, t36247: F, t35979: F, t4044: F, t212: F, t3076: F, t672: F, t678: F, t7186: F, t7294: F, t7299: F) -> (F, F, F, F, F, F, F, F) {
    let t36983 = t638 * t3899 * t271 * t641;
    let t36998 = t739 * t36293;
    let t37000 = t739 * t36247;
    let t37006 = t4044 * t35979;
    let t37017 = t672 * t212 * t3076 * t678;
    let t37053 = 0.89430439388620083049e-2 * t7186;
    let t37082 = 0.487802396665200453e-2 * t7294;
    let t37083 = 0.11709622077411463733e-2 * t7299;
    (t36983, t36998, t37000, t37006, t37017, t37053, t37082, t37083)
}
