//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1795/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1795(t10192: f64, t10194: f64, t10415: f64, t10416: f64, t1310: f64, t1315: f64, t13207: f64, t13435: f64, t2320: f64, t2328: f64, t2372: f64, t3813: f64, t3821: f64, t4151: f64, t46126: f64, t46129: f64, t46137: f64, t46233: f64, t46349: f64, t47632: f64, t47634: f64, t47648: f64, t47662: f64, t47676: f64, t47681: f64, t47687: f64, t508: f64, t511: f64, t649: f64, t651: f64, t671: f64, t94: f64) -> f64 {
    let t47692 = -8.0_f64 * t46126 * t671 - 6.0_f64 * t94 * t46137 * t508 - 4.0_f64 * t649 * t13207 - 2.0_f64 * t651 * t508 * t46233 - 24.0_f64 * t13435 * t2372 - 12.0_f64 * t10416 * t2372 + 6.0_f64 * t3821 * t4151 - 12.0_f64 * t2328 * t3813 - 12.0_f64 * t46129 * t508 - 24.0_f64 * t10194 * t1310 - 4.0_f64 * t10415 * t1310 - 6.0_f64 * t2320 * t3813 + 4.0_f64 * t1315 * t10192 + t511 * (t46349 + t47632 + t47634 + t47648 + t47662 + t47676 + t47681 + t47687);
    t47692
}
