//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2751/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2751(t10760: f64, t40763: f64, t4353: f64, t1559: f64, t775: f64, t40834: f64, t854: f64, t14587: f64, t2735: f64, t40798: f64, t826: f64, t14547: f64, t14676: f64, t14894: f64, t2745: f64, t36833: f64, t4364: f64, t50560: f64, t50573: f64, t50577: f64, t50579: f64, t50582: f64, t50586: f64, t50590: f64, t50594: f64, t50598: f64, t50600: f64, t50602: f64, t50605: f64, t50607: f64, t50608: f64, t837: f64) -> (f64, f64) {
    let t50611 = t10760 * t40763 * t4353;
    let t50613 = t1559 * t775;
    let t50615 = t40834 * t854 * t50613;
    let t50619 = t2735 * t40798 * t826 * t14587;
    let t50621 = -0.64311027177104605458e-3_f64 * t2745 * t36833 * t50560 * t837 - 0.38586616306262763275e-2_f64 * t14894 * t4364 * t14676 * t14547 - 0.22869001264178397702e-3_f64 * t50573 - 0.85748036236139473944e-4_f64 * t50577 + 0.30011812682648815881e-2_f64 * t50579 + t50582 - 0.17149607247227894789e-3_f64 * t50586 - 0.85748036236139473944e-4_f64 * t50590 + 0.15246000842785598468e-3_f64 * t50594 + 0.22869001264178397701e-3_f64 * t50598 - 0.24009450146119052704e-1_f64 * t50600 - 0.12004725073059526352e-1_f64 * t50602 - t50605 - t50607 + 0.68026775414003982663e-1_f64 * t50608 + 0.81322168495418382223e-4_f64 * t50611 - 0.30492001685571196935e-4_f64 * t50615 - 0.15246000842785598468e-4_f64 * t50619;
    (t50613, t50621)
}
