//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 954/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk954<F: Float>(t2248: F, t9120: F, t1341: F, t6909: F, t2194: F, t3370: F, t1327: F, t2198: F, t2201: F, t1353: F, t2263: F, t1365: F, t2264: F, t2284: F, t2296: F, t2318: F, t2326: F, t3416: F, t3436: F, t6972: F, t9077: F, t9084: F, t9114: F, t9117: F, t9119: F) -> (F, F, F, F, F, F, F) {
    let t9122 = 0.16081979498692535067e2 * t9120 * t2248;
    let t9124 = 1.0 * t6909 * t1341;
    let t9126 = 2.0 * t2194 * t3370;
    let t9127 = t1327 * t2198;
    let t9129 = 2.0 * t9127 * t2201;
    let t9130 = t1353 * t2263;
    let t9133 = 0.5848223622634646207e0 * t3416 * t2318 + 0.17315859105681463759e2 * t9077 * t2326 + 0.5848223622634646207e0 * t6972 * t1365 + 0.11696447245269292414e1 * t2296 * t3436 - 2.0 * t9084 * t2264 - t9114 - t9117 - t9119 - t9122 - t9124 - t9126 + t9129 + 6.0 * t2284 * t9130;
    (t9122, t9124, t9126, t9127, t9129, t9130, t9133)
}
