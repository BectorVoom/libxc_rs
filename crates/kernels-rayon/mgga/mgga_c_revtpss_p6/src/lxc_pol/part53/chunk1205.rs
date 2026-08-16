//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1205/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1205(t5795: f64, t8611: f64, t1459: f64, t34012: f64, t1916: f64, t32375: f64, t1518: f64, t572: f64, t670: f64, t8460: f64, t32374: f64, t4292: f64) -> (f64, f64, f64, f64, f64) {
    let t127449 = 6.0_f64 * t5795 * t8611;
    let t127453 = 6.0_f64 * t1459 * t34012;
    let t127455 = 6.0_f64 * t1916 * t32375;
    let t127459 = 6.0_f64 * t572 * t670 * t8460 * t1518;
    let t127462 = 6.0_f64 * t572 * t32374 * t4292;
    (t127449, t127453, t127455, t127459, t127462)
}
