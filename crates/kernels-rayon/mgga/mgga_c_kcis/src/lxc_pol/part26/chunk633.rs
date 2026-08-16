//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 633/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk633(t1378: f64, t7086: f64, t286: f64, t1368: f64, t1930: f64, t1934: f64, t1940: f64, t3969: f64, t493: f64, t500: f64, t5689: f64, t5691: f64, t5699: f64, t5719: f64, t7054: f64, t7065: f64, t7069: f64, t7073: f64, t7077: f64, t7082: f64) -> (f64, f64) {
    let t7087 = t1378 * t7086;
    let t7088 = t286 * t7087;
    let t7091 = 11.0_f64 / 108.0_f64 * t7054 * t500 - t5689 / 54.0_f64 - t5691 * t1934 / 54.0_f64 + t1930 * t1940 / 18.0_f64 - t3969 + t5699 / 432.0_f64 - t5719 / 144.0_f64 + t1368 * t7065 / 216.0_f64 - t1368 * t7069 / 144.0_f64 - t1368 * t7073 / 144.0_f64 + t1368 * t7077 / 288.0_f64 + t493 * t7082 / 48.0_f64 - t493 * t7088 / 96.0_f64;
    (t7087, t7091)
}
