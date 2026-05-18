//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 585/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk585<F: Float>(t571: F, t6041: F, t1534: F, t2042: F, t1533: F, t6006: F, t6008: F, t6013: F, t6017: F, t6021: F, t6023: F, t6025: F, t6030: F, t6032: F, t6035: F, t6039: F) -> (F, F, F, F) {
    let t6042 = t571 * t6041;
    let t6044 = t2042 * t1534;
    let t6045 = t1533 * t6044;
    let t6047 = -t6006 / F::new(6.0) - t6008 / F::new(192.0) - t6013 / F::new(128.0) - t6017 / F::new(16.0) - t6021 / F::new(256.0) + t6023 / F::new(24.0) + t6025 / F::new(48.0) + t6030 / F::new(8.0) + t6032 / F::new(24.0) - t6035 / F::new(48.0) - t6039 / F::new(16.0) - t6042 / F::new(9.0) + t6045 / F::new(6.0);
    (t6042, t6044, t6045, t6047)
}
