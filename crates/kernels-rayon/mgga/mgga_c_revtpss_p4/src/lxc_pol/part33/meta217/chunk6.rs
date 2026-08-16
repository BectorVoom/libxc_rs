//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 999/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk999(t116: f64, t1518: f64, t670: f64, t117: f64, t4292: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t5795: f64) -> (f64, f64, f64, f64) {
    let t5801 = t116 * t1518;
    let t5802 = t5801 * t670;
    let t5805 = t117 * t4292;
    let t5808 = 3.0_f64 * t1459 * t1918 + 3.0_f64 * t1461 * t1916 + 6.0_f64 * t572 * t5802 + 3.0_f64 * t572 * t5805 + t573 * t5795;
    (t5801, t5802, t5805, t5808)
}
