//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1083/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1083(t1493: f64, t36: f64, t606: f64, t119457: f64, t60221: f64, t8435: f64, t13272: f64, t32141: f64, t33612: f64, t644: f64, t8621: f64, t6972: f64) -> (f64, f64, f64, f64, f64) {
    let t125279 = t1493 * t36 * t606;
    let t125280 = t119457 * t125279;
    let t125283 = t60221 * t8435;
    let t125286 = t13272 * t32141;
    let t125290 = t8621 * t33612 * t644;
    let t125294 = t8621 * t33612 * t6972;
    (t125280, t125283, t125286, t125290, t125294)
}
