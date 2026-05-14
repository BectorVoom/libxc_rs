//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1163/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1163<F: Float>(t178: F, t5943: F, t752: F, t466: F, t779: F, t2104: F, t2107: F, t5589: F, t735: F, t154: F, t276: F, t277: F, t4932: F, t5612: F, t771: F, t299: F, t301: F, t4902: F) -> (F, F, F, F, F, F, F) {
    let t17864 = t752 * t5943 * t178;
    let t17867 = t466 * t779;
    let t17869 = t2104 * t17867 * t2107;
    let t17874 = t735 * t5589;
    let t17881 = 5.0 / 486.0 * t276 * t154 * t4932 * t277;
    let t17897 = t771 * t5612;
    let t17902 = 0.14820648238345094262e-3 * t299 * t178 * t4902 * t301;
    (t17864, t17867, t17869, t17874, t17881, t17897, t17902)
}
