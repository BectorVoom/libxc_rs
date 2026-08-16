//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2172/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2172(t2723: f64, t99315: f64, t7063: f64, t99271: f64, t7060: f64, t136: f64, t2457: f64, t7778: f64, t25299: f64, t25412: f64, t99348: f64, t25431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99369 = t99315 * t2723;
    let t99373 = t7063 * t99271;
    let t99375 = 0.25702851531048074406e-1_f64 * t99373 * t7060;
    let t99380 = t7778 * t136 * t2457;
    let t99381 = t25299 * t99380;
    let t99389 = t99348 * t25412;
    let t99391 = 0.14456046980341999104e-1_f64 * t25431 * t99389;
    (t99369, t99375, t99380, t99381, t99389, t99391)
}
