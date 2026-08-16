//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 640/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk640(t1608: f64, t6183: f64, t286: f64, t1599: f64, t1603: f64, t1612: f64, t4424: f64, t4427: f64, t4430: f64, t4439: f64, t6138: f64, t6141: f64, t6149: f64, t6152: f64, t6156: f64, t6160: f64, t6165: f64, t6169: f64, t6173: f64, t6179: f64) -> (f64, f64) {
    let t6184 = t1608 * t6183;
    let t6185 = t286 * t6184;
    let t6188 = -t6138 / 216.0_f64 - t6141 * t1603 / 216.0_f64 + t6141 * t1612 / 72.0_f64 - t4424 + t4427 / 1728.0_f64 - t4430 / 576.0_f64 + t6149 / 1728.0_f64 + t4439 * t6152 / 432.0_f64 - t4439 * t6156 / 576.0_f64 - t4439 * t6160 / 288.0_f64 - t1599 * t6165 / 288.0_f64 - t6169 / 576.0_f64 - t4439 * t6173 / 576.0_f64 + t1599 * t6179 / 96.0_f64 - t1599 * t6185 / 192.0_f64;
    (t6184, t6188)
}
