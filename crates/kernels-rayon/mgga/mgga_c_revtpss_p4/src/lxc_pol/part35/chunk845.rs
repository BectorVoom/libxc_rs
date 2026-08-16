//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 845/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk845(t3172: f64, t6634: f64, t3610: f64, t5265: f64, t5293: f64, t3153: f64, t6628: f64, t6622: f64, t1263: f64, t6587: f64, t6624: f64, t1247: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20786 = t3172 * t6634;
    let t20787 = t3610 * t20786;
    let t20789 = t5293 * t5265;
    let t20795 = t6628 * t3153;
    let t20800 = t6622 * t3153;
    let t20809 = t1263 * t6587;
    let t20816 = t3172 * t6624;
    let t20817 = t1247 * t20816;
    (t20787, t20789, t20795, t20800, t20809, t20817)
}
