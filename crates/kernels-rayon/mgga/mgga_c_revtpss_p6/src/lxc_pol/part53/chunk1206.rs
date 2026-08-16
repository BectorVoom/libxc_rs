//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1206/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1206(t26123: f64, t572: f64, t7741: f64, t28042: f64, t7330: f64, t1459: f64, t34004: f64, t2040: f64, t28271: f64, t105823: f64, t7002: f64, t7331: f64, t7944: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127465 = 12.0_f64 * t572 * t26123 * t7741;
    let t127468 = 12.0_f64 * t572 * t7330 * t28042;
    let t127472 = 6.0_f64 * t1459 * t34004;
    let t127475 = t2040 * t28271;
    let t127480 = 12.0_f64 * t572 * t105823 * t7002;
    let t127481 = t7944 * t7331;
    (t127465, t127468, t127472, t127475, t127480, t127481)
}
