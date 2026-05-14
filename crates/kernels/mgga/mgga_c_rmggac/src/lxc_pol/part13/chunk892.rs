//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 892/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk892<F: Float>(t8546: F, t8549: F, t8552: F, t9341: F, t9344: F, t7430: F, t7438: F, t8090: F, t8091: F, t8093: F, t8095: F, t8096: F, t8097: F, t8098: F, t8563: F, t8565: F) -> (F, F, F, F, F, F) {
    let t42436 = 0.35922725105591425692e0 * t8546;
    let t42437 = 0.71845450211182851384e0 * t8549;
    let t42438 = 0.17961362552795712846e0 * t8552;
    let t42444 = 0.79828278012425390428e-1 * t9341;
    let t42445 = 0.4726e1 * t9344;
    let t42446 = t8090 + t8091 - 0.79453919800822633544e-4 * t7430 + t8093 + 0.23836175940246790064e-3 * t7438 + t42444 - t8095 - t42445 + t8096 + t8097 + t8098;
    let t42450 = 0.5454932330849068346e-1 * t8563;
    let t42451 = 0.13637330827122670865e-1 * t8565;
    (t42436, t42437, t42438, t42446, t42450, t42451)
}
