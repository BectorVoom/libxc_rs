//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1501/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1501(t118822: f64, t118864: f64, t118911: f64, t118955: f64, t117: f64, t118630: f64, t118749: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t22544: f64, t22556: f64, t31475: f64, t31494: f64, t31497: f64, t31500: f64, t31711: f64, t31728: f64, t35858: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5805: f64, t5883: f64, t670: f64, t6941: f64, t6948: f64, t8320: f64, t8336: f64, t8343: f64, t8421: f64, t8427: f64, t8430: f64, param_d: f64) -> (f64, f64) {
    let t118957 = t118822 + t118864 + t118911 + t118955;
    let t118962 = 6.0_f64 * t572 * t118630 * t670 + 12.0_f64 * t1916 * t31500 + 6.0_f64 * t2207 * t22556 + 3.0_f64 * t572 * t117 * t118749 + 3.0_f64 * t31711 * t1461 + 12.0_f64 * t1916 * t31497 + 3.0_f64 * t8336 * t6948 + 6.0_f64 * t8421 * t5805 + 12.0_f64 * t1916 * t31494 + 6.0_f64 * t6941 * t8343 + 6.0_f64 * t31475 * t1918 + 6.0_f64 * t5795 * t8430 + 6.0_f64 * t572 * t5883 * t8320 + 12.0_f64 * t1459 * t31728 + 3.0_f64 * t22544 * t2209 + 12.0_f64 * t572 * t35858 * t4292 + param_d * t118957 * t573 + 12.0_f64 * t5795 * t8427;
    (t118957, t118962)
}
