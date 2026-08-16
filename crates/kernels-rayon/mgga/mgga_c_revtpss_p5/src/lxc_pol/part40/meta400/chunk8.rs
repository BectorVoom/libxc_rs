//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1472/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1472(t1459: f64, t1461: f64, t18190: f64, t18204: f64, t18208: f64, t18211: f64, t18214: f64, t1916: f64, t1918: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64) -> f64 {
    let t18217 = 12.0_f64 * t1459 * t5802 + 6.0_f64 * t1459 * t5805 + 6.0_f64 * t1461 * t5795 + t18190 * t573 + 6.0_f64 * t18204 * t572 + 12.0_f64 * t18208 * t572 + 6.0_f64 * t18211 * t572 + 3.0_f64 * t18214 * t572 + 6.0_f64 * t1916 * t4162 + 3.0_f64 * t1916 * t4165 + 3.0_f64 * t1918 * t4158;
    t18217
}
