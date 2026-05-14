//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 825/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk825<F: Float>(t1533: F, t6044: F, t6006: F, t6008: F, t6013: F, t6017: F, t6021: F, t6023: F, t6025: F, t6030: F, t6032: F, t6035: F, t6039: F, t6042: F, t6005: F, t1506: F) -> (F, F, F) {
    let t6045 = t1533 * t6044;
    let t6047 = -t6006 / 6.0 - t6008 / 192.0 - t6013 / 128.0 - t6017 / 16.0 - t6021 / 256.0 + t6023 / 24.0 + t6025 / 48.0 + t6030 / 8.0 + t6032 / 24.0 - t6035 / 48.0 - t6039 / 16.0 - t6042 / 9.0 + t6045 / 6.0;
    let t6048 = t6005 + t6047;
    let t6049 = t1506 * t6048;
    (t6045, t6048, t6049)
}
