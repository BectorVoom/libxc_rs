//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2751/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2751<F: Float>(t10760: F, t40763: F, t4353: F, t1559: F, t775: F, t40834: F, t854: F, t14587: F, t2735: F, t40798: F, t826: F, t14547: F, t14676: F, t14894: F, t2745: F, t36833: F, t4364: F, t50560: F, t50573: F, t50577: F, t50579: F, t50582: F, t50586: F, t50590: F, t50594: F, t50598: F, t50600: F, t50602: F, t50605: F, t50607: F, t50608: F, t837: F) -> (F, F) {
    let t50611 = t10760 * t40763 * t4353;
    let t50613 = t1559 * t775;
    let t50615 = t40834 * t854 * t50613;
    let t50619 = t2735 * t40798 * t826 * t14587;
    let t50621 = -F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t36833 * t50560 * t837 - F::cast_from(0.38586616306262763275e-2_f64) * t14894 * t4364 * t14676 * t14547 - F::cast_from(0.22869001264178397702e-3_f64) * t50573 - F::cast_from(0.85748036236139473944e-4_f64) * t50577 + F::cast_from(0.30011812682648815881e-2_f64) * t50579 + t50582 - F::cast_from(0.17149607247227894789e-3_f64) * t50586 - F::cast_from(0.85748036236139473944e-4_f64) * t50590 + F::cast_from(0.15246000842785598468e-3_f64) * t50594 + F::cast_from(0.22869001264178397701e-3_f64) * t50598 - F::cast_from(0.24009450146119052704e-1_f64) * t50600 - F::cast_from(0.12004725073059526352e-1_f64) * t50602 - t50605 - t50607 + F::cast_from(0.68026775414003982663e-1_f64) * t50608 + F::cast_from(0.81322168495418382223e-4_f64) * t50611 - F::cast_from(0.30492001685571196935e-4_f64) * t50615 - F::cast_from(0.15246000842785598468e-4_f64) * t50619;
    (t50613, t50621)
}
