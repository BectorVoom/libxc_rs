//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 776/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk776(t1538: f64, t2042: f64, t571: f64, t1534: f64, t1533: f64, t6006: f64, t6008: f64, t6013: f64, t6017: f64, t6021: f64, t6023: f64, t6025: f64, t6030: f64, t6032: f64, t6035: f64, t6039: f64) -> (f64, f64, f64, f64, f64) {
    let t6041 = t2042 * t1538;
    let t6042 = t571 * t6041;
    let t6044 = t2042 * t1534;
    let t6045 = t1533 * t6044;
    let t6047 = -t6006 / 6.0_f64 - t6008 / 192.0_f64 - t6013 / 128.0_f64 - t6017 / 16.0_f64 - t6021 / 256.0_f64 + t6023 / 24.0_f64 + t6025 / 48.0_f64 + t6030 / 8.0_f64 + t6032 / 24.0_f64 - t6035 / 48.0_f64 - t6039 / 16.0_f64 - t6042 / 9.0_f64 + t6045 / 6.0_f64;
    (t6041, t6042, t6044, t6045, t6047)
}
