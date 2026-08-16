//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 719/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk719(t160: f64, t8593: f64, t2315: f64, t8581: f64, t648: f64, t8590: f64, t4620: f64, t15: f64, t2317: f64, t2320: f64, t2444: f64, t2448: f64, t650: f64, t720: f64, t8573: f64, t8574: f64, t8578: f64, t8585: f64) -> (f64, f64, f64, f64) {
    let t8594 = t8593 * t160;
    let t8596 = t2315 * t8581;
    let t8598 = t648 * t8590;
    let t8601 = -0.26426666666666666667e-1_f64 * t8594 + 0.17617777777777777778e-1_f64 * t8596 - 0.20554074074074074074e-1_f64 * t8598 - 0.12841111111111111111e-1_f64 * t4620;
    let t8604 = -t8573 * t8574 / 3.0_f64 - t8578 * t2317 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t2444 * t8581 - t8585 * t650 / 4.0_f64 + t2448 * t2320 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t720 * t8590 + t15 * t8601 / 2.0_f64;
    (t8594, t8596, t8598, t8604)
}
