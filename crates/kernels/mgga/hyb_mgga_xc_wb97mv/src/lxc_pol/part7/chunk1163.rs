//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1163/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1163<F: Float>(t1226: F, t19: F, t3003: F, t8195: F, t8436: F, t8172: F, t1175: F, t6395: F, t6388: F, t222: F, t22469: F, t3: F, t2968: F, t6381: F, t2003: F, t3135: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24832 = t19 * t3003 * t1226;
    let t24834 = t8195 * t8436;
    let t24836 = t8195 * t8172;
    let t24838 = t1175 * t6395;
    let t24840 = t1175 * t6388;
    let t24843 = t3 * t22469 * t222;
    let t24844 = t24843 * t2968;
    let t24846 = t1175 * t6381;
    let t24849 = t19 * t2003 * t3135;
    (t24832, t24834, t24836, t24838, t24840, t24843, t24844, t24846, t24849)
}
