//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 483/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk483(t1120: f64, t1715: f64, t128: f64, t1119: f64, t422: f64, t1118: f64) -> (f64, f64, f64, f64, f64) {
    let t1716 = t1120 * t1715;
    let t1717 = t128 * t1716;
    let t1719 = -t1119 + 0.17808333333333333333e-1_f64 * t1717;
    let t1721 = 0.621814e-1_f64 * t1719 * t422;
    let t1723 = -t1118 / 3.0_f64 + t1717 / 3.0_f64;
    (t1716, t1717, t1719, t1721, t1723)
}
