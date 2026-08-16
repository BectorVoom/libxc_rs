//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 826/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk826(t1214: f64, t1263: f64, t1122: f64, t1042: f64, t1209: f64, t1284: f64, t3624: f64) -> (f64, f64, f64, f64) {
    let t3712 = t1263 * t1214;
    let t3713 = t3712 * t1122;
    let t3714 = t1042 * t3713;
    let t3717 = t1209 * t1284;
    let t3718 = t3717 * t3624;
    (t3713, t3714, t3717, t3718)
}
