//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1084/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1084(t1493: f64, t640: f64, t8621: f64, t4237: f64, t84: f64, t1470: f64, t644: f64, t8442: f64, t119457: f64, t36: f64, t606: f64, t60221: f64, t8435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125244 = t8621 * t640 * t1493;
    let t125248 = t8621 * t84 * t4237;
    let t125260 = t1470 * t644;
    let t125261 = t8442 * t125260;
    let t125268 = t1470 * t640;
    let t125269 = t119457 * t125268;
    let t125279 = t1493 * t36 * t606;
    let t125280 = t119457 * t125279;
    let t125283 = t60221 * t8435;
    (t125244, t125248, t125261, t125269, t125280, t125283)
}
