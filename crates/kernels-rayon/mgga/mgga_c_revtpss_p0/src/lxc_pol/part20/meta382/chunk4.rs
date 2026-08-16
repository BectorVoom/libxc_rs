//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1392/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1392(t2786: f64, t40921: f64, t10073: f64, t10654: f64, t10666: f64, t10952: f64, t2815: f64, t40326: f64, t40491: f64, t40537: f64, t40888: f64, t40894: f64, t40902: f64, t40914: f64, t40918: f64, t4366: f64, t4504: f64, t4514: f64, t820: f64, t837: f64, t879: f64) -> f64 {
    let t40922 = t40921 * t2786;
    let t40924 = t10073 * t10654;
    let t40926 = 0.15805078039045227836e2_f64 * t4504 * t40888 * t4366 + 0.21951497276451705328e-1_f64 * t40894 - 0.26341796731742046395e1_f64 * t820 * t2815 * t10666 - 0.19756347548806534796e1_f64 * t820 * t879 * t40491 + 0.15805078039045227836e2_f64 * t820 * t40902 * t40326 - 0.23707617058567841754e2_f64 * t820 * t10952 * t40537 - 0.79025390195226139184e1_f64 * t4514 * t40888 * t837 + 0.65854491829355115985e-1_f64 * t40914 - 0.13170898365871023197e0_f64 * t40918 + 0.68293547082294194357e-1_f64 * t40922 - 0.7805952431506226415e-2_f64 * t40924;
    t40926
}
