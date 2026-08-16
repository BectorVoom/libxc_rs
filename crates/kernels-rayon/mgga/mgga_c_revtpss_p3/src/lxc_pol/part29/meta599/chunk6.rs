//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2045/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2045(t28993: f64, t571: f64, t101724: f64, t104041: f64, t104054: f64, t1458: f64, t1464: f64, t18178: f64, t18217: f64, t1921: f64, t2111: f64, t2118: f64, t26704: f64, t28945: f64, t3: f64, t4154: f64, t4168: f64, t575: f64, t8114: f64, t8130: f64, t95182: f64, t95184: f64, t95186: f64, t95190: f64) -> f64 {
    let t104062 = 2.0_f64 * t571 * t28993;
    let t104065 = t8114 * t4168 + 2.0_f64 * t95190 + t95186 + 2.0_f64 * t95182 + t2111 * t18217 + t18178 * t2118 + t1458 * (t101724 + t104054) + t26704 * t1921 + t4154 * t8130 + 2.0_f64 * t28945 * t1464 + t104062 + t3 * t104041 * t575 + t95184;
    t104065
}
