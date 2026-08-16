//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1925/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1925(t99033: f64, t99041: f64, t99066: f64, t99069: f64, t99073: f64, t99077: f64, t99085: f64, t99099: f64, t99102: f64, t136: f64, t2457: f64, t8015: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t103296 = 0.80031500487063509014e-2_f64 * t99033;
    let t103301 = 0.22866142996303859718e-3_f64 * t99041;
    let t103315 = 0.16006300097412701803e0_f64 * t99066;
    let t103316 = 0.11433071498151929859e-3_f64 * t99069;
    let t103318 = 0.2032800112371413129e-2_f64 * t99073;
    let t103320 = 0.10164000561857065645e-3_f64 * t99077;
    let t103324 = 0.2032800112371413129e-3_f64 * t99085;
    let t103336 = 7.0_f64 / 36.0_f64 * t99099;
    let t103337 = 7.0_f64 / 12.0_f64 * t99102;
    let t103363 = t8015 * t136 * t2457;
    (t103296, t103301, t103315, t103316, t103318, t103320, t103324, t103336, t103337, t103363)
}
