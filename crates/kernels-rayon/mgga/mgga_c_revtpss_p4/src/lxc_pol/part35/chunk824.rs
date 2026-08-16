//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 824/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk824(t10777: f64, t18643: f64, t251: f64, t5977: f64, t1558: f64, t1568: f64, t233: f64, t6041: f64, t869: f64, t689: f64, t6016: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18644 = t10777 * t18643;
    let t18677 = t251 * t5977;
    let t18681 = t1568 * t1558;
    let t18688 = t233 * t6041;
    let t18689 = t869 * t18688;
    let t18690 = t689 * t18689;
    let t18699 = t251 * t6016;
    let t18714 = t822 * t6041;
    (t18644, t18677, t18681, t18690, t18699, t18714)
}
