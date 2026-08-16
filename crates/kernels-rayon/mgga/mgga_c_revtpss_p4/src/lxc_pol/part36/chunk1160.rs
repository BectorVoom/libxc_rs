//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1160/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1160(t26865: f64, t4890: f64, t3767: f64, t3782: f64, t1243: f64, t8190: f64, t1811: f64, t3140: f64, t1276: f64, t2148: f64, t1032: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29096 = t26865 * t4890;
    let t29097 = t3767 * t29096;
    let t29100 = t3782 * t29096;
    let t29122 = t1243 * t8190;
    let t29127 = t1811 * t3140;
    let t29129 = t2148 * t29127 * t1276;
    let t29135 = t1811 * t1032;
    (t29096, t29097, t29100, t29122, t29129, t29135)
}
