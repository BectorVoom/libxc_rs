//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1047/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1047<F: Float>(t10252: F, t2099: F, t3235: F, t2411: F, t9795: F, t10189: F, t2029: F, t10225: F, t18657: F, t2380: F, t10097: F, t3185: F, t926: F, t8319: F, t8392: F, t10044: F, t8467: F) -> (F, F, F, F, F, F, F) {
    let t27083 = t3235 * t2099 * t10252;
    let t27085 = t2411 * t9795;
    let t27104 = t10189 * t2029;
    let t27119 = t2380 * t18657 * t10225;
    let t27122 = t3185 * t926 * t10097;
    let t27151 = t8319 * t8392;
    let t27153 = t10044 * t8467;
    (t27083, t27085, t27104, t27119, t27122, t27151, t27153)
}
