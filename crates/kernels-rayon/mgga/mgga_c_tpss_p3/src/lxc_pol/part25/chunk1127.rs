//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1127/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1127(t13330: f64, t4283: f64, t3931: f64, t242: f64, t5064: f64, t9523: f64, t1125: f64, t1130: f64, t15485: f64, t15489: f64, t15493: f64, t15500: f64, t15504: f64, t15506: f64, t4248: f64, t4258: f64, t4265: f64, t4280: f64, t9535: f64) -> f64 {
    let t15510 = t4283 * t13330;
    let t15511 = t3931 * t15510;
    let t15515 = t242 * t9523 * t5064;
    let t15516 = t1125 * t15515;
    let t15518 = -t15485 / 432.0_f64 + t15489 / 2304.0_f64 + t9535 - 19.0_f64 / 2592.0_f64 * t15493 * t1130 - t4258 * t4248 / 288.0_f64 - t15500 / 3456.0_f64 - t15504 / 6912.0_f64 + 19.0_f64 / 2592.0_f64 * t15506 - 5.0_f64 / 1296.0_f64 * t4265 * t4280 - t1125 * t15511 / 2304.0_f64 + 5.0_f64 / 20736.0_f64 * t15516;
    t15518
}
