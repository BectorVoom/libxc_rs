//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1046/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1046(t6173: f64, t954: f64, t2970: f64, t6157: f64, t2974: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t324: f64, t1633: f64) -> (f64, f64, f64, f64, f64) {
    let t6174 = t6173 * t954;
    let t6177 = t6157 * t2970;
    let t6184 = t2974 + 0.61805555555555555556e-2_f64 * t4571 - 0.61805555555555555555e-2_f64 * t6094 + 0.18541666666666666667e-1_f64 * t6098 - 0.92708333333333333333e-2_f64 * t6102;
    let t6185 = t6184 * t324;
    let t6189 = t1633 * t1633;
    (t6174, t6177, t6184, t6185, t6189)
}
