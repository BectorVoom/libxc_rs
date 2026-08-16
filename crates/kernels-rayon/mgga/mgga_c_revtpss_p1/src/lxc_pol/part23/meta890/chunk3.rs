//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2834/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2834(t23281: f64, t2652: f64, t14785: f64, t23148: f64, t2477: f64, t2745: f64, t40607: f64, t40611: f64, t4433: f64, t50607: f64, t50608: f64, t50611: f64, t50615: f64, t50619: f64, t50634: f64, t50681: f64, t6017: f64, t61833: f64, t61839: f64, t76572: f64, t76583: f64, t76587: f64, t76591: f64, t775: f64, t828: f64, t851: f64) -> f64 {
    let t76593 = t2652 * t23281;
    let t76595 = -t50607 + 0.68026775414003982664e-1_f64 * t50608 + 0.24396650548625514667e-3_f64 * t50611 - 0.30492001685571196934e-4_f64 * t50615 - 0.15246000842785598467e-4_f64 * t50619 + 0.68026775414003982663e-1_f64 * t50634 - 0.12862205435420921092e-1_f64 * t2745 * t14785 * t6017 * t4433 - 0.15246000842785598467e-3_f64 * t61833 - 0.42874018118069736973e-4_f64 * t76572 + t40607 - t40611 - 0.6098400337114239387e-4_f64 * t61839 - 0.81312004494856525158e-3_f64 * t50681 + 0.42874018118069736972e-2_f64 * t851 * t2477 * t828 * t23148 * t775 - 0.42874018118069736973e-4_f64 * t76583 + 0.17149607247227894789e-3_f64 * t76587 + 0.7623000421392799234e-4_f64 * t76591 - 0.60023625365297631763e-1_f64 * t76593;
    t76595
}
