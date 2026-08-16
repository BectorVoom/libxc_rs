//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1220/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1220(t34382: f64, t4254: f64, t1936: f64, t29337: f64, t651: f64, t32822: f64, t7937: f64, t28177: f64, t8764: f64, t34399: f64, t7239: f64, t125512: f64, t125514: f64, t125515: f64, t125517: f64, t125521: f64, t125522: f64, t2007: f64, t29422: f64, t7221: f64, t8152: f64) -> f64 {
    let t129332 = t4254 * t34382;
    let t129335 = t651 * t29337 * t1936;
    let t129339 = t32822 * t7937;
    let t129342 = t8764 * t28177;
    let t129344 = t34399 * t7239;
    let t129346 = -t2007 * t29422 - t7221 * t8152 + t125512 - t125514 - 2.0_f64 * t125515 - 2.0_f64 * t125517 - t125521 - t125522 - 2.0_f64 * t129332 - 2.0_f64 * t129335 - t129339 + 3.0_f64 * t129342 + 3.0_f64 * t129344;
    t129346
}
