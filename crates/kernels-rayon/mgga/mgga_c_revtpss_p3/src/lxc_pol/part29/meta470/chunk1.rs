//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1733/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1733(t25299: f64, t26519: f64, t25365: f64, t7407: f64, t1956: f64, t213: f64, t25383: f64, t257: f64, t26437: f64, t26439: f64, t26441: f64, t26448: f64, t26475: f64, t26483: f64, t26486: f64, t26489: f64, t26493: f64, t26498: f64, t26500: f64, t26502: f64, t26508: f64, t26511: f64, t26515: f64, t7067: f64, t7070: f64, t7415: f64, t7424: f64) -> (f64, f64, f64) {
    let t26521 = 0.17135234354032049604e-2_f64 * t25299 * t26519;
    let t26522 = t25365 * t7407;
    let t26524 = -t26437 + t26439 + 0.8673628188205199462e0_f64 * t7070 * t26441 + 0.17347256376410398924e1_f64 * t25383 * t7415 - 0.10975748638225852664e-1_f64 * t26448 - 0.4336814094102599731e0_f64 * t1956 * t26475 - 0.8673628188205199462e0_f64 * t7067 * t7424 + 0.25702851531048074406e-1_f64 * t26483 + 0.51405703062096148812e-1_f64 * t26486 - 0.26020884564615598386e1_f64 * t7070 * t26489 + 0.17347256376410398924e1_f64 * t7070 * t26493 - 0.19514881078765566038e-1_f64 * t26498 - 0.14456046980341999104e-1_f64 * t26500 + 0.65854491829355115987e0_f64 * t213 * t26502 * t257 + t26508 - 0.8673628188205199462e0_f64 * t7070 * t26511 + 0.4336814094102599731e0_f64 * t7070 * t26515 + t26521 - 0.25702851531048074406e-1_f64 * t26522;
    (t26521, t26522, t26524)
}
