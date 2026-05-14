//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 445/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk445<F: Float>(t179: F, t2068: F, t655: F, t299: F, t771: F, t775: F, t52: F, t779: F, t754: F, t768: F, t46: F, t752: F) -> (F, F, F, F, F) {
    let t2070 = t179 * t2068 * t655;
    let t2071 = t299 * t2070;
    let t2085 = t771 * t775;
    let t2089 = t52 * t779;
    let t2094 = t768 * t754;
    let t2095 = t2094 * t46;
    let t2096 = t752 * t2095;
    (t2071, t2085, t2089, t2094, t2096)
}
