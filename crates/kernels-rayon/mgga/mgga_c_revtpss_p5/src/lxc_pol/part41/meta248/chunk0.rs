//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 942/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk942(t1312: f64, t1518: f64, t4248: f64, t5877: f64, t5883: f64, t5920: f64, t93: f64, t5545: f64, t5547: f64, t5570: f64, t5572: f64, t1907: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6773 = 2.0_f64 * t1312 * t5920 + 4.0_f64 * t1518 * t4248 + 2.0_f64 * t5883 * t93 + t5877;
    let t6777 = 8.0_f64 * t5545;
    let t6778 = 8.0_f64 * t5547;
    let t6779 = 2.0_f64 * t5570;
    let t6780 = 0.11696447245269292414e1_f64 * t5572;
    let t6781 = t1907 * t1907;
    (t6773, t6777, t6778, t6779, t6780, t6781)
}
