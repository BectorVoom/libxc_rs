//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1955/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1955(t2106: f64, t47672: f64, t2028: f64, t28911: f64, t25894: f64, t97680: f64, t25875: f64, t96236: f64, t97688: f64, t26304: f64, t97705: f64, t96187: f64, t97685: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102070 = t2106 * t47672;
    let t102078 = t2028 * t28911;
    let t102081 = 0.28912093960683998208e-1_f64 * t25894 * t102078 * t97680;
    let t102084 = 0.51405703062096148812e-1_f64 * t25875 * t102078 * t97680;
    let t102086 = 0.51405703062096148812e-1_f64 * t96236 * t97688;
    let t102087 = t2028 * t26304;
    let t102090 = 0.14456046980341999104e-1_f64 * t25894 * t102087 * t97705;
    let t102093 = 0.25702851531048074406e-1_f64 * t25875 * t102087 * t97705;
    let t102096 = 0.28912093960683998208e-1_f64 * t96187 * t97685;
    (t102070, t102081, t102084, t102086, t102090, t102093, t102096)
}
