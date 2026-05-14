//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 506/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk506<F: Float>(t2193: F, t2196: F, t2203: F, t2242: F, t2250: F, t2254: F, t2257: F, t2262: F, t2264: F, t2279: F, t2284: F, t2287: F, t2294: F, t2296: F, t2301: F, t2303: F, t2318: F, t2323: F, t2326: F, t271: F, t820: F, t829: F, t839: F, t848: F) -> (F,) {
    let t2329 = -0.310907e-1 * t2254 * t271 + 2.0 * t2257 * t829 - 2.0 * t2262 * t2264 + 1.0 * t820 * t2279 + 0.32163958997385070134e2 * t2284 * t2287 + t2193 - t2196 + t2203 - t2242 - t2250 - 0.19751673498613801407e-1 * t2294 + 0.11696447245269292414e1 * t2296 * t848 - 0.11696447245269292414e1 * t2301 * t2303 + 0.5848223622634646207e0 * t839 * t2318 + 0.17315859105681463759e2 * t2323 * t2326;
    (t2329,)
}
