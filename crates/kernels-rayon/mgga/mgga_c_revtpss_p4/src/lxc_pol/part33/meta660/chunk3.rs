//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2140/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2140(t25317: f64, t25383: f64, t29644: f64, t4533: f64, t7070: f64, t7769: f64, t7770: f64, t93378: f64, t93382: f64, t93384: f64, t93391: f64, t99303: f64, t99485: f64, t99487: f64, t99491: f64, t99493: f64, t99496: f64, t99502: f64, t99520: f64, t99522: f64) -> f64 {
    let t106477 = -0.34270468708064099208e-2_f64 * t93378 - t99485 - t99487 - 0.65049603595885220126e-3_f64 * t93382 - 0.96373646535613327357e-2_f64 * t93384 - t99491 + t99493 + 0.19274729307122665472e-1_f64 * t99496 - 0.26020884564615598386e1_f64 * t25383 * t29644 - t99502 - 0.52041769129231196772e1_f64 * t7070 * t25317 * t7769 * t4533 - 0.13009920719177044025e-2_f64 * t99520 + 0.17347256376410398924e1_f64 * t99303 * t7770 - 0.34270468708064099208e-1_f64 * t99522 + 0.73171657588172351096e-2_f64 * t93391;
    t106477
}
