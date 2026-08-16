//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1488/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1488(t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t31475: f64, t31494: f64, t31497: f64, t31500: f64, t31506: f64, t31509: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t8336: f64, t8343: f64, t8346: f64, t8421: f64, t8427: f64, t8430: f64) -> f64 {
    let t31512 = 6.0_f64 * t1459 * t8427 + 3.0_f64 * t1459 * t8430 + 3.0_f64 * t1461 * t8421 + 6.0_f64 * t1916 * t8343 + 3.0_f64 * t1916 * t8346 + 3.0_f64 * t1918 * t8336 + 6.0_f64 * t2207 * t5802 + 3.0_f64 * t2207 * t5805 + 3.0_f64 * t2209 * t5795 + t31475 * t573 + 6.0_f64 * t31494 * t572 + 6.0_f64 * t31497 * t572 + 6.0_f64 * t31500 * t572 + 6.0_f64 * t31506 * t572 + 3.0_f64 * t31509 * t572;
    t31512
}
