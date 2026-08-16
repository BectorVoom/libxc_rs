//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2171/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2171(t2470: f64, t27340: f64, t25387: f64, t1580: f64, t25317: f64, t25391: f64, t25392: f64, t25394: f64, t27316: f64, t27349: f64, t7070: f64, t886: f64, t92864: f64, t93186: f64, t93276: f64, t93278: f64, t93283: f64, t93286: f64, t99334: f64, t99342: f64, t99344: f64, t99346: f64, t99351: f64, t99360: f64) -> (f64, f64) {
    let t99365 = t27340 * t2470;
    let t99366 = t25387 * t99365;
    let t99368 = -0.17347256376410398924e1_f64 * t25391 * t99334 * t25394 - 0.17347256376410398924e1_f64 * t25391 * t92864 * t27349 + t99342 - t93276 - t99344 + t99346 + t93278 + t99351 + 0.43368140941025997312e-1_f64 * t93283 - 0.52041769129231196772e1_f64 * t7070 * t25317 * t27316 * t886 - 0.65854491829355115987e0_f64 * t93186 * t1580 - 0.17347256376410398924e1_f64 * t25391 * t25392 * t99360 + 0.38549458614245330943e-1_f64 * t93286 - 0.34270468708064099208e-1_f64 * t99366;
    (t99365, t99368)
}
