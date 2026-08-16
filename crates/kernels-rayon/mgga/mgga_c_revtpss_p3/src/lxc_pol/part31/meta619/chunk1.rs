//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2068/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2068(t14991: f64, t93261: f64, t25296: f64, t27213: f64, t92843: f64, t98815: f64, t27291: f64, t689: f64, t25431: f64, t25411: f64, t2453: f64, t27212: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99228 = t93261 * t14991;
    let t99231 = 0.14456046980341999104e-1_f64 * t27213 * t25296;
    let t99234 = 0.28912093960683998208e-1_f64 * t92843 * t98815;
    let t99241 = t27291 * t689;
    let t99243 = 0.14456046980341999104e-1_f64 * t25431 * t99241;
    let t99245 = 0.25702851531048074406e-1_f64 * t25411 * t99241;
    let t99257 = t2453 * t27212;
    (t99228, t99231, t99234, t99243, t99245, t99257)
}
