//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1187/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1187(t30066: f64, t30109: f64, t532: f64, t1450: f64, t2014: f64, t1868: f64, t1907: f64, t8717: f64, t25082: f64, t7732: f64, t7742: f64, t1936: f64, t6765: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30110 = t30066 + t30109;
    let t30111 = t532 * t30110;
    let t30112 = t30111 * t1450;
    let t30113 = t2014 * t30112;
    let t30122 = t1868 * t1907;
    let t30123 = t8717 * t30122;
    let t30125 = 6.0_f64 * t25082 * t30123;
    let t30127 = 4.0_f64 * t7732 * t7742;
    let t30128 = t6765 * t1936;
    (t30110, t30111, t30112, t30113, t30122, t30123, t30125, t30127, t30128)
}
