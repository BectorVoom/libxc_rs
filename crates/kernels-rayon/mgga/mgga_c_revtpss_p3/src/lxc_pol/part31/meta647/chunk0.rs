//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2124/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2124(t4481: f64, t99285: f64, t212: f64, t29636: f64, t689: f64, t780: f64, t105944: f64, t1955: f64, t106178: f64, t1558: f64, t231: f64, t25317: f64, t25383: f64, t25416: f64, t2723: f64, t27265: f64, t27275: f64, t27353: f64, t27357: f64, t29610: f64, t29643: f64, t29669: f64, t62593: f64, t7070: f64, t7076: f64, t7079: f64, t7779: f64, t886: f64, t93118: f64, t93231: f64, t93242: f64, t99287: f64, t99297: f64) -> (f64, f64) {
    let t106267 = t99285 * t4481;
    let t106272 = t689 * t212 * t29636 * t780;
    let t106275 = t1955 * t105944;
    let t106284 = -t99287 - t93231 + 0.8673628188205199462e0_f64 * t25383 * t29669 + 0.8673628188205199462e0_f64 * t7070 * t7076 * t27265 * t1558 * t231 - 0.52041769129231196772e1_f64 * t7070 * t25317 * t29610 * t886 - 0.26020884564615598386e1_f64 * t27353 * t27357 * t62593 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t29643 * t886 - 0.19514881078765566037e-1_f64 * t106267 - 0.4818682326780666368e-3_f64 * t99297 - 0.54878743191129263322e-2_f64 * t106272 + 0.24093411633903331839e-3_f64 * t93242 + 0.4336814094102599731e0_f64 * t106275 * t7079 - 0.8673628188205199462e0_f64 * t7070 * t25416 * t106178 * t2723 - 0.8673628188205199462e0_f64 * t27275 * t7779;
    (t106275, t106284)
}
