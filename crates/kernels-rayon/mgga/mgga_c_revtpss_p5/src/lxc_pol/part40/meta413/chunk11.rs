//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1505/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1505(t2212: f64, t5789: f64, t117151: f64, t117153: f64, t117155: f64, t117161: f64, t118106: f64, t118108: f64, t118110: f64, t118154: f64, t118198: f64, t1456: f64, t1458: f64, t1914: f64, t31244: f64, t31512: f64, t4154: f64, t5790: f64, t8349: f64, t8433: f64) -> f64 {
    let t118203 = 2.0_f64 * t5789 * t2212;
    let t118204 = t4154 * t8433 + t1914 * t31244 + 2.0_f64 * t5790 * t8349 + t118106 + t118108 + t118110 + 2.0_f64 * t117161 + 2.0_f64 * t1456 * t31512 + t1458 * (t118154 + t118198) + t117155 + 2.0_f64 * t117153 + t117151 + t118203;
    t118204
}
