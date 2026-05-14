//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 531/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk531<F: Float>(t771: F, t775: F, t52: F, t779: F, t154: F, t1885: F, t754: F, t768: F, t46: F, t752: F) -> (F, F, F, F, F) {
    let t2085 = t771 * t775;
    let t2089 = t52 * t779;
    let t2091 = t154 * t2089 * t1885;
    let t2094 = t768 * t754;
    let t2095 = t2094 * t46;
    let t2096 = t752 * t2095;
    (t2085, t2089, t2091, t2094, t2096)
}
