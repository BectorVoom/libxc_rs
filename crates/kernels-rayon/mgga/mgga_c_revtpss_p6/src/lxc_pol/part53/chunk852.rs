//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 852/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk852(t3173: f64, t7122: f64, t1007: f64, t7106: f64, t1968: f64, t3080: f64, t7105: f64, t800: f64, t3244: f64, t7111: f64, t3111: f64, t7132: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25529 = t7122 * t3173;
    let t25535 = t7106 * t1007;
    let t25538 = t1968 * t3080 / 432.0_f64;
    let t25539 = t7105 * t800;
    let t25543 = t7111 * t3244;
    let t25551 = t7132 * t3111;
    (t25529, t25535, t25538, t25539, t25543, t25551)
}
