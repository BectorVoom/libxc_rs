//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1817/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1817(t28986: f64, t670: f64, t117: f64, t28683: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t28956: f64, t28975: f64, t28978: f64, t28981: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t7547: f64, t7554: f64, t7557: f64, t8118: f64, t8124: f64, t8127: f64) -> (f64, f64, f64) {
    let t28987 = t28986 * t670;
    let t28990 = t117 * t28683;
    let t28993 = 6.0_f64 * t1459 * t8124 + 3.0_f64 * t1459 * t8127 + 3.0_f64 * t1461 * t8118 + 6.0_f64 * t1916 * t7554 + 3.0_f64 * t1916 * t7557 + 3.0_f64 * t1918 * t7547 + 6.0_f64 * t2113 * t5802 + 3.0_f64 * t2113 * t5805 + 3.0_f64 * t2115 * t5795 + t28956 * t573 + 6.0_f64 * t28975 * t572 + 6.0_f64 * t28978 * t572 + 6.0_f64 * t28981 * t572 + 6.0_f64 * t28987 * t572 + 3.0_f64 * t28990 * t572;
    (t28987, t28990, t28993)
}
