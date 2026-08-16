//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1222/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1222(t125525: f64, t125531: f64, t125532: f64, t125536: f64, t129354: f64, t129357: f64, t129366: f64, t129371: f64, t1518: f64, t2322: f64, t32791: f64, t34394: f64, t34431: f64, t4254: f64, t4292: f64, t651: f64, t670: f64, t8756: f64) -> f64 {
    let t129372 = -2.0_f64 * t1518 * t32791 * t651 - 2.0_f64 * t34394 * t651 * t670 - 2.0_f64 * t4292 * t651 * t8756 - 2.0_f64 * t2322 * t34431 - 2.0_f64 * t34431 * t4254 - t125525 - t125531 - 3.0_f64 * t125532 + t125536 + 2.0_f64 * t129354 - 2.0_f64 * t129357 - 3.0_f64 * t129366 + t129371;
    t129372
}
