//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 742/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk742(t1017: f64, t1028: f64, t1047: f64, t1068: f64, t348: f64, t375: f64, t7106: f64, t7110: f64, t7111: f64, t7114: f64, t7117: f64, t7122: f64, t7126: f64, t7130: f64, t7132: f64) -> f64 {
    let t7135 = -t7106 * t348 / 36.0_f64 + t7110 + t7111 * t1017 / 288.0_f64 + 0.42874018118069736972e-3_f64 * t7114 * t375 - 0.42874018118069736972e-3_f64 * t7117 * t1028 + 0.42874018118069736972e-3_f64 * t7122 * t1047 - 0.22866142996303859718e-2_f64 * t7126 * t375 + t7130 + 0.28582678745379824648e-3_f64 * t7132 * t1068;
    t7135
}
