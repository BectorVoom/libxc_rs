//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 959/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk959<F: Float>(t15297: F, t3908: F, t5384: F, t310: F, t5375: F, t323: F, t3242: F, t545: F, t1352: F, t3770: F, t1095: F, t3037: F, t3210: F, t398: F, t513: F) -> (F, F, F, F, F) {
    let t15299 = t5384 * t15297 * t3908;
    let t15303 = t310 * t5375;
    let t15314 = t3242 * t545 * t323;
    let t15337 = t3770 * t1352;
    let t15348 = t3210 * t398 * t1095 * t513 * t3037;
    (t15299, t15303, t15314, t15337, t15348)
}
