//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2083/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2083(t25526: f64, t4820: f64, t15769: f64, t25522: f64, t15687: f64, t25515: f64, t3317: f64, t25525: f64, t4878: f64, t27450: f64, t3173: f64, t16035: f64, t25580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100048 = t25526 * t4820;
    let t100051 = 0.3811023832717309953e-3_f64 * t25522 * t15769;
    let t100054 = t25515 * t15687;
    let t100055 = t3317 * t100054;
    let t100074 = t4878 * t25525;
    let t100078 = 0.57165357490759649296e-3_f64 * t27450 * t3173;
    let t100092 = 0.57165357490759649296e-3_f64 * t25580 * t16035;
    (t100048, t100051, t100054, t100055, t100074, t100078, t100092)
}
