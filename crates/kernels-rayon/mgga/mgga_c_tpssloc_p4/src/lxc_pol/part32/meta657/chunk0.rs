//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2086/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2086(t87405: f64, t87432: f64, t87653: f64, t87666: f64, t87718: f64, t87779: f64, t87898: f64, t87915: f64, t90503: f64, t90551: f64, t90582: f64, t90642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92677 = 0.10541775202358879834e-2_f64 * t87405;
    let t92689 = 0.22608743412718618878e-1_f64 * t87432;
    let t92781 = 0.16449340668482264365e-1_f64 * t87653;
    let t92794 = 0.12793931631041761173e0_f64 * t87666;
    let t92817 = 0.10417915756705434098e0_f64 * t87718;
    let t92863 = 0.16449340668482264365e-1_f64 * t87779;
    let t92954 = 0.52089578783527170489e-1_f64 * t87898;
    let t92961 = 0.16449340668482264365e-1_f64 * t87915;
    let t93335 = 0.12793931631041761173e0_f64 * t90503;
    let t93368 = 0.10417915756705434098e0_f64 * t90551;
    let t93387 = 0.52089578783527170489e-1_f64 * t90582;
    let t93438 = 0.16449340668482264365e-1_f64 * t90642;
    (t92677, t92689, t92781, t92794, t92817, t92863, t92954, t92961, t93335, t93368, t93387, t93438)
}
