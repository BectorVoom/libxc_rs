//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2223/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2223(t1469: f64, t1925: f64, t603: f64, t4186: f64, t77: f64, t84: f64, t2242: f64, t5826: f64, t19680: f64, t108733: f64, t108737: f64, t108745: f64, t108749: f64, t108753: f64, t1928: f64, t25099: f64, t25106: f64, t29544: f64, t29548: f64, t6958: f64, t6960: f64) -> f64 {
    let t108757 = t603 * t1469 * t1925;
    let t108759 = t77 * t84 * t4186;
    let t108762 = t2242 * t5826;
    let t108765 = t603 * t19680;
    let t108768 = 5.0_f64 / 3.0_f64 * t25099 * t29544 + 5.0_f64 / 3.0_f64 * t25106 * t29544 + 5.0_f64 / 3.0_f64 * t6958 * t108733 + 5.0_f64 / 3.0_f64 * t6958 * t108737 + 5.0_f64 / 6.0_f64 * t25099 * t29548 + 5.0_f64 / 6.0_f64 * t25106 * t29548 + 5.0_f64 / 6.0_f64 * t6958 * t108745 + 5.0_f64 / 6.0_f64 * t6958 * t108749 - 5.0_f64 / 3.0_f64 * t108753 * t6960 + 2.0_f64 / 3.0_f64 * t108757 * t108759 + t108762 * t1928 / 3.0_f64 + t108765 * t1928 / 3.0_f64;
    t108768
}
