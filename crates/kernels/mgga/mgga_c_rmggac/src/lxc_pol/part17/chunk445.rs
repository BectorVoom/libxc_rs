//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 445/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk445<F: Float>(t574: F, t934: F, t1710: F, t3869: F, t312: F, t50: F, t537: F, t814: F, t1714: F, t809: F, t3878: F, t90: F, t1726: F, t3885: F, t316: F, t547: F) -> (F, F, F, F, F, F, F, F) {
    let t5757 = t934 * t574;
    let t5799 = t3869 * t1710;
    let t5800 = t5799 * t312;
    let t5803 = t537 * t50;
    let t5804 = t5803 * t814;
    let t5809 = t809 * t1714;
    let t5810 = t5809 * t312;
    let t5814 = -t814 - 3.0 * t3878;
    let t5815 = t90 * t5814;
    let t5824 = t3885 * t1726;
    let t5825 = t5824 * t316;
    let t5828 = t547 * t50;
    (t5757, t5800, t5804, t5810, t5814, t5815, t5825, t5828)
}
