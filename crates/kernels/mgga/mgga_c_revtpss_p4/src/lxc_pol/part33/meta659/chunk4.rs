//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2132/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2132<F: Float>(t4481: F, t99285: F, t212: F, t29636: F, t689: F, t780: F, t105944: F, t1955: F, t106178: F, t1558: F, t231: F, t25317: F, t25383: F, t25416: F, t2723: F, t27265: F, t27275: F, t27353: F, t27357: F, t29610: F, t29643: F, t29669: F, t62593: F, t7070: F, t7076: F, t7079: F, t7779: F, t886: F, t93118: F, t93231: F, t93242: F, t99287: F, t99297: F) -> (F, F) {
    let t106267 = t99285 * t4481;
    let t106272 = t689 * t212 * t29636 * t780;
    let t106275 = t1955 * t105944;
    let t106284 = -t99287 - t93231 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t29669 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t27265 * t1558 * t231 - F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t29610 * t886 - F::cast_from(0.26020884564615598386e1_f64) * t27353 * t27357 * t62593 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t29643 * t886 - F::cast_from(0.19514881078765566037e-1_f64) * t106267 - F::cast_from(0.4818682326780666368e-3_f64) * t99297 - F::cast_from(0.54878743191129263322e-2_f64) * t106272 + F::cast_from(0.24093411633903331839e-3_f64) * t93242 + F::cast_from(0.4336814094102599731e0_f64) * t106275 * t7079 - F::cast_from(0.8673628188205199462e0_f64) * t7070 * t25416 * t106178 * t2723 - F::cast_from(0.8673628188205199462e0_f64) * t27275 * t7779;
    (t106275, t106284)
}
