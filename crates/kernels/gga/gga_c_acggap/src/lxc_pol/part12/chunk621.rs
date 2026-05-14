//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 621/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk621<F: Float>(t469: F, t922: F, t104: F, t566: F, t95: F, t839: F, t130: F, t595: F, t154: F) -> (F, F, F, F, F) {
    let t7288 = t469 * t922;
    let t7297 = t566 * t95 * t104;
    let t7301 = t469 * t839;
    let t7309 = t130 * t595;
    let t7310 = t7309 * t154;
    (t7288, t7297, t7301, t7309, t7310)
}
