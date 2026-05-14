//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 844/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk844<F: Float>(t10568: F, t5396: F, t760: F, t755: F, t10641: F, t1964: F, t5399: F, t763: F, t1670: F, t4761: F, t4787: F, t10690: F, t591: F, t10696: F, t1961: F, t5397: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12002 = 0.53272592592592592592e-1 * t10568;
    let t12017 = 1.0 / t5396 / t760;
    let t12018 = t755 * t12017;
    let t12042 = 0.16068111111111111111e1 * t10568;
    let t12043 = 0.46308888888888888888e0 * t10641;
    let t12058 = 1.0 / t5396 / t1964;
    let t12059 = t755 * t12058;
    let t12061 = 1.0 / t5399 / t763;
    let t12084 = t1670 * t4761;
    let t12095 = t1670 * t4787;
    let t12098 = t591 * t10690;
    let t12105 = t591 * t10696;
    let t12114 = t1961 * t5397;
    (t12002, t12018, t12042, t12043, t12059, t12061, t12084, t12095, t12098, t12105, t12114)
}
