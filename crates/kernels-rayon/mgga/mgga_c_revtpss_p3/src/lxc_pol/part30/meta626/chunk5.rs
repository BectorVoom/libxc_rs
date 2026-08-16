//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2173/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2173(t1568: f64, t786: f64, t25410: f64, t25413: f64, t14587: f64, t25383: f64, t25391: f64, t2718: f64, t27189: f64, t27287: f64, t27292: f64, t27300: f64, t27312: f64, t27313: f64, t27353: f64, t27357: f64, t2829: f64, t51574: f64, t7048: f64, t92864: f64, t92917: f64, t93297: f64, t93304: f64, t99369: f64, t99375: f64, t99381: f64, t99391: f64) -> (f64, f64) {
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99406 = 0.14456046980341999104e-1_f64 * t99404 * t25413;
    let t99409 = 0.17347256376410398924e1_f64 * t25391 * t27357 * t99369 - t99375 - 0.26020884564615598386e1_f64 * t27353 * t27357 * t51574 + 0.17135234354032049604e-2_f64 * t99381 + 0.8673628188205199462e0_f64 * t25383 * t27287 + 0.8673628188205199462e0_f64 * t25383 * t27292 - 0.52041769129231196772e1_f64 * t25383 * t27300 - t99391 - 0.25702851531048074406e-1_f64 * t93297 - 0.17347256376410398924e1_f64 * t27353 * t2718 * t7048 * t14587 - 0.17347256376410398924e1_f64 * t92917 * t27313 - 0.17347256376410398924e1_f64 * t25391 * t92864 * t27312 - 0.25702851531048074406e-1_f64 * t93304 - t99406 - 0.65854491829355115987e0_f64 * t27189 * t2829;
    (t99403, t99409)
}
