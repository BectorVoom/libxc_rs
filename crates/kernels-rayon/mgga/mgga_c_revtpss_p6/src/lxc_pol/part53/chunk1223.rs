//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1223/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1223(t34399: f64, t7313: f64, t28166: f64, t8763: f64, t28168: f64, t125537: f64, t125539: f64, t125541: f64, t125543: f64, t125545: f64, t125547: f64, t125550: f64, t125552: f64, t125554: f64, t2163: f64, t28160: f64, t7683: f64, t7725: f64) -> f64 {
    let t129376 = t34399 * t7313;
    let t129377 = t8763 * t28166;
    let t129378 = t129377 * t28168;
    let t129391 = -t2163 * t28160 - t7683 * t7725 - 2.0_f64 * t125537 - 2.0_f64 * t125539 - 2.0_f64 * t125541 - 2.0_f64 * t125543 - 2.0_f64 * t125545 - 2.0_f64 * t125547 - 2.0_f64 * t125550 - 2.0_f64 * t125552 - 2.0_f64 * t125554 + t129376 + 6.0_f64 * t129378;
    t129391
}
