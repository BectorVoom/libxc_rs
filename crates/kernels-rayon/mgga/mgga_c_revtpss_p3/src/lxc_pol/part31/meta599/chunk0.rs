//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2029/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2029(t94395: f64, t97688: f64, t94649: f64, t1892: f64, t786: f64, t25877: f64, t25881: f64, t2028: f64, t25931: f64, t14224: f64, t689: f64, t25894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97690 = 0.28912093960683998208e-1_f64 * t94395 * t97688;
    let t97698 = 0.51405703062096148812e-1_f64 * t94649 * t97688;
    let t97699 = t786 * t1892;
    let t97700 = t97699 * t25877;
    let t97702 = 0.28912093960683998208e-1_f64 * t97700 * t25881;
    let t97703 = t2028 * t25931;
    let t97705 = t14224 * t689;
    let t97707 = 0.14456046980341999104e-1_f64 * t25894 * t97703 * t97705;
    (t97690, t97698, t97699, t97700, t97702, t97703, t97705, t97707)
}
