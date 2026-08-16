//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1158/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1158(t2066: f64, t5752: f64, t1395: f64, t7329: f64, t7332: f64, t4123: f64, t7318: f64, t28594: f64, t8191: f64, t7338: f64, t7948: f64, t29434: f64, t29436: f64, t29438: f64, t29440: f64, t29442: f64, t29444: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29446 = t5752 * t2066;
    let t29448 = t1395 * t7329;
    let t29450 = t1395 * t7332;
    let t29452 = t4123 * t7318;
    let t29454 = t28594 * t8191;
    let t29456 = t7948 * t7338;
    let t29458 = t29434 / 8.0_f64 - t29436 / 128.0_f64 - t29438 / 12.0_f64 + t29440 / 48.0_f64 + t29442 / 64.0_f64 + t29444 / 12.0_f64 - t29446 / 48.0_f64 - 19.0_f64 / 72.0_f64 * t29448 + t29450 / 9.0_f64 - t29452 / 64.0_f64 + t29454 / 3.0_f64 - t29456 / 12.0_f64;
    (t29446, t29448, t29450, t29452, t29454, t29456, t29458)
}
