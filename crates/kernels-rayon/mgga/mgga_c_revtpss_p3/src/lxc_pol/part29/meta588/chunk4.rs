//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1945/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1945(t26205: f64, t7709: f64, t101129: f64, t101132: f64, t101139: f64, t101337: f64, t101782: f64, t101783: f64, t101785: f64, t101790: f64, t2048: f64, t25159: f64, t26175: f64, t28116: f64, t28119: f64, t7352: f64, t7706: f64, t95310: f64) -> f64 {
    let t101793 = t7709 * t26205;
    let t101805 = 10.0_f64 * t26175 * t101337 + t101782 - 440.0_f64 / 27.0_f64 * t101783 + 10.0_f64 * t101785 * t25159 + t101790 + 10.0_f64 / 3.0_f64 * t95310 * t7706 - 176.0_f64 / 27.0_f64 * t101793 - 2.0_f64 / 3.0_f64 * t101129 * t2048 - 4.0_f64 / 3.0_f64 * t101132 * t2048 - 4.0_f64 / 3.0_f64 * t28116 * t7352 - 2.0_f64 / 3.0_f64 * t101139 * t2048 - 4.0_f64 / 3.0_f64 * t28119 * t7352;
    t101805
}
