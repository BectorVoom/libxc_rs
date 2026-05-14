//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 959/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk959<F: Float>(t1364: F, t6965: F, t2302: F, t6968: F, t3460: F, t6960: F, t2303: F, t2341: F, t260: F, t3447: F, t855: F, t9023: F, t9122: F, t9124: F, t9126: F, t9129: F, t9189: F, t9191: F, t9193: F, t9196: F, t9199: F, t9202: F, t9206: F, t9209: F, t9213: F) -> (F, F, F, F, F) {
    let t9229 = t6965 * t1364;
    let t9230 = t6968 * t2302;
    let t9231 = t9229 * t9230;
    let t9234 = t3460 * t6960;
    let t9237 = t3460 * t2303;
    let t9240 = t9122 + t9124 + t9126 - t9129 + 0.19751673498613801407e-1 * t260 * t9023 - 0.5848223622634646207e0 * t3447 * t2341 - 0.10254018858216406658e4 * t855 * t9231 - 0.17315859105681463759e2 * t855 * t9234 - 0.35089341735807877242e1 * t855 * t9237 + t9189 - t9191 + t9193 - t9196 - t9199 - t9202 + t9206 + t9209 + t9213;
    (t9230, t9231, t9234, t9237, t9240)
}
