//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 963/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk963(t1252: f64, t1797: f64, t26873: f64, t26880: f64, t29010: f64, t29020: f64, t29023: f64, t29027: f64, t29052: f64, t29079: f64, t29107: f64, t5270: f64, t5279: f64, t5287: f64, t5299: f64, t5304: f64, t7618: f64, t7624: f64) -> f64 {
    let t29109 = 0.28582678745379824648e-3_f64 * t26880 * t5299 - 0.57165357490759649296e-3_f64 * t7624 * t5270 + 0.42874018118069736972e-3_f64 * t29010 * t1252 + 0.28582678745379824648e-3_f64 * t26880 * t5279 + 0.42874018118069736972e-3_f64 * t26873 * t1797 + 0.42874018118069736972e-3_f64 * t7618 * t5287 - 0.22866142996303859718e-2_f64 * t29020 * t1252 + 0.28582678745379824648e-3_f64 * t29023 + 0.47637797908966374413e-3_f64 * t7624 * t5304 - t29027 / 108.0_f64 + t29052 + t29079 + t29107;
    t29109
}
