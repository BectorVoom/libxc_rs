//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2072/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2072(t25387: f64, t99349: f64, t2470: f64, t27340: f64, t7063: f64, t99271: f64, t7060: f64, t136: f64, t2457: f64, t7778: f64, t25299: f64, t25412: f64, t99348: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99351 = 0.51405703062096148812e-1_f64 * t25387 * t99349;
    let t99365 = t27340 * t2470;
    let t99366 = t25387 * t99365;
    let t99373 = t7063 * t99271;
    let t99375 = 0.25702851531048074406e-1_f64 * t99373 * t7060;
    let t99380 = t7778 * t136 * t2457;
    let t99381 = t25299 * t99380;
    let t99389 = t99348 * t25412;
    (t99351, t99365, t99366, t99375, t99380, t99381, t99389)
}
