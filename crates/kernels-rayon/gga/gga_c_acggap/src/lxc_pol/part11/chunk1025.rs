//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1025/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1025(t1181: f64, t2068: f64, t20935: f64, t604: f64, t30269: f64, t30297: f64, t34072: f64, t34074: f64, t34076: f64, t34077: f64, t34078: f64, t34082: f64, t34085: f64, t34089: f64, t34092: f64, t34095: f64, t34100: f64, t34102: f64, t34105: f64, t34107: f64, t34111: f64) -> f64 {
    let t34115 = t2068 * t1181 * t604 * t20935;
    let t34117 = 0.94344276868812456204e-2_f64 * t30269 - 0.68598428988911579156e-2_f64 * t34072 + 0.34299214494455789578e-2_f64 * t34074 + t34076 + t34077 - 0.34299214494455789578e-2_f64 * t34078 - 0.21437009059034868486e-2_f64 * t30297 - t34082 - 0.15724046144802076034e-2_f64 * t34085 - 0.10718504529517434243e-2_f64 * t34089 + t34092 - 0.62896184579208304136e-3_f64 * t34095 - t34100 + t34102 - 0.94344276868812456204e-2_f64 * t34105 + 0.94344276868812456204e-2_f64 * t34107 + 0.10718504529517434243e-2_f64 * t34111 + 0.42874018118069736972e-3_f64 * t34115;
    t34117
}
