//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1952/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1952(t28154: f64, t95296: f64, t28147: f64, t95319: f64, t28150: f64, t7348: f64, t25162: f64, t101200: f64, t101204: f64, t101230: f64, t101234: f64, t101252: f64, t101399: f64, t26175: f64, t26182: f64, t28628: f64, t92565: f64, t95276: f64, t95306: f64, t95316: f64, t95340: f64) -> f64 {
    let t101955 = 160.0_f64 / 9.0_f64 * t28154 * t95296;
    let t101969 = 160.0_f64 / 3.0_f64 * t95319 * t28147;
    let t101970 = t7348 * t28150;
    let t101972 = 160.0_f64 / 9.0_f64 * t25162 * t101970;
    let t101975 = 10.0_f64 / 3.0_f64 * t28154 * t95306 - 20.0_f64 * t101252 * t95340 - t101955 + 20.0_f64 * t95276 * t28147 + 20.0_f64 * t26175 * t101399 + 20.0_f64 * t26175 * t101200 + 10.0_f64 * t26175 * t101204 + 20.0_f64 / 3.0_f64 * t101230 * t26182 - 70.0_f64 * t95316 * t101234 - t101969 - t101972 + 20.0_f64 / 3.0_f64 * t92565 * t28628;
    t101975
}
