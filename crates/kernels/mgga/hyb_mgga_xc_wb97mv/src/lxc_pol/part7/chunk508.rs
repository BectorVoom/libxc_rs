//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 508/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk508<F: Float>(t2300: F, t2302: F, t847: F, t2317: F, t838: F, t2322: F, t2325: F, t2193: F, t2196: F, t2203: F, t2242: F, t2250: F, t2294: F, t2329: F, t2333: F, t260: F, t855: F, t857: F) -> (F, F, F, F) {
    let t2337 = t2300 * t2302 * t847;
    let t2341 = t838 * t2317 * t847;
    let t2344 = t2322 * t2302;
    let t2345 = t2344 * t2325;
    let t2348 = -t2193 + t2196 - t2203 + t2242 + t2250 + t260 * t2329 + 0.19751673498613801407e-1 * t260 * t2294 - 0.11696447245269292414e1 * t2333 * t857 + 0.11696447245269292414e1 * t855 * t2337 - 0.5848223622634646207e0 * t855 * t2341 - 0.17315859105681463759e2 * t855 * t2345;
    (t2337, t2341, t2345, t2348)
}
