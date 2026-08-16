//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1464/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1464(t21660: f64, t22531: f64, t3: f64, t5883: f64, t670: f64, t4292: f64, t5801: f64, t116: f64, t5920: f64, t117: f64, t21881: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t6941: f64, t6945: f64, t6948: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22532 = t21660 + t22531;
    let t22533 = t3 * t22532;
    let t22544 = param_d * t22532;
    let t22556 = t670 * t5883;
    let t22559 = t5801 * t4292;
    let t22564 = t116 * t5920;
    let t22565 = t22564 * t670;
    let t22568 = t117 * t21881;
    let t22571 = 6.0_f64 * t1459 * t6945 + 3.0_f64 * t1459 * t6948 + 3.0_f64 * t1461 * t6941 + 12.0_f64 * t1916 * t5802 + 6.0_f64 * t1916 * t5805 + 6.0_f64 * t1918 * t5795 + t22544 * t573 + 6.0_f64 * t22556 * t572 + 12.0_f64 * t22559 * t572 + 6.0_f64 * t22565 * t572 + 3.0_f64 * t22568 * t572;
    (t22533, t22544, t22556, t22559, t22565, t22568, t22571)
}
