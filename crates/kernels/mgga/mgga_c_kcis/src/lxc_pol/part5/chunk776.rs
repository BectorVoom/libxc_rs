//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 776/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk776<F: Float>(t1538: F, t2042: F, t571: F, t1534: F, t1533: F, t6006: F, t6008: F, t6013: F, t6017: F, t6021: F, t6023: F, t6025: F, t6030: F, t6032: F, t6035: F, t6039: F) -> (F, F, F, F, F) {
    let t6041 = t2042 * t1538;
    let t6042 = t571 * t6041;
    let t6044 = t2042 * t1534;
    let t6045 = t1533 * t6044;
    let t6047 = -t6006 / F::cast_from(6.0_f64) - t6008 / F::cast_from(192.0_f64) - t6013 / F::cast_from(128.0_f64) - t6017 / F::cast_from(16.0_f64) - t6021 / F::cast_from(256.0_f64) + t6023 / F::cast_from(24.0_f64) + t6025 / F::cast_from(48.0_f64) + t6030 / F::cast_from(8.0_f64) + t6032 / F::cast_from(24.0_f64) - t6035 / F::cast_from(48.0_f64) - t6039 / F::cast_from(16.0_f64) - t6042 / F::cast_from(9.0_f64) + t6045 / F::cast_from(6.0_f64);
    (t6041, t6042, t6044, t6045, t6047)
}
