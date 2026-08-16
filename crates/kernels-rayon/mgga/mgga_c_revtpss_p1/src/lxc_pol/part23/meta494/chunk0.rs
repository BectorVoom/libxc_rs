//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1974/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1974(t1151: f64, t20629: f64, t16835: f64, t1733: f64, t5063: f64, t5105: f64, t12361: f64, t6439: f64, t3379: f64, t6471: f64, t12429: f64, t12470: f64, t17032: f64, t20606: f64, t20609: f64, t20612: f64, t20615: f64, t20619: f64, t20622: f64, t20626: f64, t3452: f64, t3477: f64, t5147: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20631 = 1.0_f64 * t20629 * t1151;
    let t20633 = 2.0_f64 * t16835 * t1733;
    let t20635 = 2.0_f64 * t5063 * t5105;
    let t20637 = 2.0_f64 * t12361 * t6439;
    let t20639 = 1.0_f64 * t3379 * t6471;
    let t20640 = 0.64327917994770140268e2_f64 * t17032 * t5147 + 6.0_f64 * t3477 * t20606 - 4.0_f64 * t3452 * t20609 - 0.19298375398431042081e3_f64 * t12429 * t20612 - 2.0_f64 * t3452 * t20615 + 0.32163958997385070134e2_f64 * t3477 * t20619 + 0.64327917994770140268e2_f64 * t3477 * t20622 + 0.2069040516770936012e4_f64 * t12470 * t20626 - t20631 - t20633 - t20635 + t20637 - t20639;
    (t20631, t20633, t20635, t20637, t20639, t20640)
}
