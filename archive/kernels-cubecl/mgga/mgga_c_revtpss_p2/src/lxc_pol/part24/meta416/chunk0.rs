//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1361/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1361<F: Float>(t43043: F, t4891: F, t3057: F, t3298: F, t11773: F, t11926: F, t11858: F, t15688: F, t12077: F, t15905: F, t994: F, t11725: F, t828: F) -> (F, F, F, F, F, F) {
    let t43044 = t43043 * t4891;
    let t43049 = t3057 * t3298;
    let t43050 = t43049 * t4891;
    let t43069 = t11926 * t11773;
    let t43082 = t11858 * t15688;
    let t43105 = t994 * t12077 * t15905;
    let t43131 = t828 * t11725;
    (t43044, t43050, t43069, t43082, t43105, t43131)
}
