//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2476/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2476(t48299: f64, t123: f64, t2630: f64, t5566: f64, t13665: f64, t9863: f64, t9866: f64, t47101: f64, t9575: f64, t9572: f64, t1320: f64, t13680: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48300 = 0.51947577317044391276e2_f64 * t48299;
    let t48302 = t5566 * t123 * t2630;
    let t48303 = 0.32530743900905219526e-1_f64 * t48302;
    let t48304 = t13665 * t9863;
    let t48306 = t13665 * t9866;
    let t48312 = 96.0_f64 * t47101;
    let t48313 = t13665 * t9575;
    let t48324 = t13665 * t9572;
    let t48326 = t1320 * t13680;
    (t48300, t48303, t48304, t48306, t48312, t48313, t48324, t48326)
}
