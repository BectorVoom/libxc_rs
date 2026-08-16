//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2831/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2831(t10811: f64, t23297: f64, t14923: f64, t23336: f64, t14894: f64, t18525: f64, t40455: f64, t40489: f64, t4364: f64, t50436: f64, t50443: f64, t50454: f64, t50457: f64, t50505: f64, t50524: f64, t61689: f64, t61692: f64, t61697: f64, t61699: f64, t61718: f64, t61727: f64, t61754: f64, t76284: f64) -> f64 {
    let t76500 = t10811 * t23297;
    let t76502 = t14923 * t23336;
    let t76517 = 0.85748036236139473945e-3_f64 * t61689 - 0.17149607247227894789e-3_f64 * t61692 - 0.76230004213927992338e-3_f64 * t61697 + 0.13605355082800796532e0_f64 * t61699 + 0.3001181268264881588e-2_f64 * t76500 + 0.24009450146119052705e-1_f64 * t76502 - 0.77173232612525526552e-2_f64 * t14894 * t4364 * t76284 * t18525 - 0.60246173160355784831e-6_f64 * t50436 + 0.45732285992607719436e-3_f64 * t50443 - t50454 + 0.27107389498472794075e-3_f64 * t50457 + 0.76230004213927992336e-4_f64 * t61718 + 0.15246000842785598468e-2_f64 * t61727 - 0.80328230880474379776e-6_f64 * t40455 + 0.72250660161932334527e-3_f64 * t40489 - t50505 + 0.12004725073059526352e0_f64 * t61754 + 0.34013387707001991332e0_f64 * t50524;
    t76517
}
