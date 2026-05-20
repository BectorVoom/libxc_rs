//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1392/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1392<F: Float>(t2786: F, t40921: F, t10073: F, t10654: F, t10666: F, t10952: F, t2815: F, t40326: F, t40491: F, t40537: F, t40888: F, t40894: F, t40902: F, t40914: F, t40918: F, t4366: F, t4504: F, t4514: F, t820: F, t837: F, t879: F) -> F {
    let t40922 = t40921 * t2786;
    let t40924 = t10073 * t10654;
    let t40926 = F::cast_from(0.15805078039045227836e2_f64) * t4504 * t40888 * t4366 + F::cast_from(0.21951497276451705328e-1_f64) * t40894 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t2815 * t10666 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t879 * t40491 + F::cast_from(0.15805078039045227836e2_f64) * t820 * t40902 * t40326 - F::cast_from(0.23707617058567841754e2_f64) * t820 * t10952 * t40537 - F::cast_from(0.79025390195226139184e1_f64) * t4514 * t40888 * t837 + F::cast_from(0.65854491829355115985e-1_f64) * t40914 - F::cast_from(0.13170898365871023197e0_f64) * t40918 + F::cast_from(0.68293547082294194357e-1_f64) * t40922 - F::cast_from(0.7805952431506226415e-2_f64) * t40924;
    t40926
}
