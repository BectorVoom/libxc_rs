//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 820/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk820<F: Float>(t25: F, t6534: F, t285: F, t2909: F, t6539: F, t1003: F, t417: F, t18443: F, t994: F, t993: F, t6533: F, t9874: F, t4966: F, t4972: F, t6517: F, t9959: F) -> (F, F, F, F, F, F) {
    let t19255 = t25 * t6534;
    let t19256 = t285 * t19255;
    let t19258 = t2909 * t6539;
    let t19259 = t19258 * t1003;
    let t19260 = t417 * t19259;
    let t19263 = t994 * t18443;
    let t19264 = t993 * t19263;
    let t19267 = t9874 * t6533;
    let t19268 = t19267 * t1003;
    let t19269 = t417 * t19268;
    let t19272 = t4966 * t4972;
    let t19273 = t417 * t19272;
    let t19278 = t9959 * t6517;
    (t19256, t19260, t19264, t19269, t19273, t19278)
}
