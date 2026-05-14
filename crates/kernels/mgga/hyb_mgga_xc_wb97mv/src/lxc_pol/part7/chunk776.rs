//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 776/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk776<F: Float>(t2300: F, t4229: F, t847: F, t4242: F, t838: F, t2322: F, t2325: F, t1373: F, t260: F, t3447: F, t4159: F, t4161: F, t4165: F, t4191: F, t4194: F, t4225: F, t4249: F, t855: F) -> (F, F, F, F, F) {
    let t4256 = t2300 * t4229 * t847;
    let t4260 = t838 * t4242 * t847;
    let t4263 = t2322 * t4229;
    let t4264 = t4263 * t2325;
    let t4267 = -t4159 + t4161 - t4165 + t4191 + t4194 + t260 * t4249 + 0.19751673498613801407e-1 * t260 * t4225 - 0.11696447245269292414e1 * t3447 * t1373 + 0.11696447245269292414e1 * t855 * t4256 - 0.5848223622634646207e0 * t855 * t4260 - 0.17315859105681463759e2 * t855 * t4264;
    (t4256, t4260, t4263, t4264, t4267)
}
