//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1181/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1181<F: Float>(t17333: F, t4293: F, t6010: F, t2035: F, t4274: F, t1529: F, t6041: F, t16658: F, t5904: F, t5903: F, t4249: F, t6038: F, t1528: F, t492: F, t5910: F, t15860: F) -> (F, F, F, F, F, F, F) {
    let t17334 = t4293 * t17333;
    let t17335 = t6010 * t17334;
    let t17337 = t2035 * t4274;
    let t17339 = t1529 * t6041;
    let t17341 = t5904 * t16658;
    let t17342 = t5903 * t17341;
    let t17344 = t4249 * t6038;
    let t17346 = t1528 * t492;
    let t17347 = t17346 * t5910;
    let t17349 = t5904 * t15860;
    (t17335, t17337, t17339, t17342, t17344, t17347, t17349)
}
