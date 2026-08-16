//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2131/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2131(t27341: f64, t99463: f64, t99466: f64, t14495: f64, t25391: f64, t27199: f64, t27287: f64, t29659: f64, t4533: f64, t7067: f64, t7070: f64, t7071: f64, t7759: f64, t93372: f64, t99414: f64, t99460: f64, t99465: f64, t99468: f64, t99472: f64, t99475: f64, t99480: f64, t99481: f64) -> f64 {
    let t106446 = t99463 * t27341;
    let t106448 = t99466 * t27341;
    let t106461 = -0.4336814094102599731e0_f64 * t7067 * t29659 + 0.91399340044406952588e-2_f64 * t99460 + 0.51405703062096148813e-1_f64 * t106446 + t99465 - t99468 - 0.28912093960683998207e-1_f64 * t106448 + 0.8673628188205199462e0_f64 * t27199 * t27287 - t99472 + t99475 + 0.22849835011101738147e-2_f64 * t93372 - t99480 - 0.19274729307122665472e-1_f64 * t99481 + 0.17347256376410398924e1_f64 * t7070 * t7071 * t7759 * t4533 - 0.17347256376410398924e1_f64 * t25391 * t99414 * t14495;
    t106461
}
