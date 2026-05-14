//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1032/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1032<F: Float>(t1375: F, t19123: F, t19109: F, t457: F, t5860: F, t960: F, t5845: F, t19136: F, t19114: F, t1383: F, t19127: F, t1471: F, t5848: F, t965: F, t5851: F, t1186: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20711 = t1375 * t19123;
    let t20714 = t457 * t19109;
    let t20718 = 0.18736e-1 * t960 * t5860;
    let t20719 = t960 * t5845;
    let t20721 = t1375 * t19136;
    let t20724 = t457 * t19114;
    let t20727 = t457 * t19123;
    let t20730 = t1383 * t19127;
    let t20733 = t1471 * t19109;
    let t20736 = t965 * t5848;
    let t20739 = 0.17611111111111111111e-2 * t965 * t5851;
    let t20740 = t1383 * t19136;
    let t20743 = t1186 * t19114;
    (t20711, t20714, t20718, t20719, t20721, t20724, t20727, t20730, t20733, t20736, t20739, t20740, t20743)
}
