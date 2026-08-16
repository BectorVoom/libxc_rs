//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 996/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk996<F: Float>(t15065: F, t5177: F, t284: F, t5082: F, t3339: F, t1800: F, t3361: F, t1170: F, t3477: F, t5096: F, t3432: F, t5172: F) -> (F, F, F, F, F) {
    let t15066 = t15065 * t5177;
    let t15068 = t5082 * t284;
    let t15069 = t15068 * t3339;
    let t15071 = t3361 * t1800;
    let t15072 = t1170 * t15071;
    let t15074 = t3477 * t5096;
    let t15076 = t5172 * t3432;
    (t15066, t15069, t15072, t15074, t15076)
}
