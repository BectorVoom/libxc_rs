//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 585/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk585<F: Float>(t2191: F, t2194: F, t2201: F, t2237: F, t2245: F, t2249: F, t2252: F, t2257: F, t2259: F, t2274: F, t2279: F, t2282: F, t2289: F, t2291: F, t2296: F, t2298: F, t2313: F, t2318: F, t2321: F, t365: F, t863: F, t872: F, t882: F, t891: F) -> (F,) {
    let t2324 = -0.310907e-1 * t2249 * t365 + 2.0 * t2252 * t872 - 2.0 * t2257 * t2259 + 1.0 * t863 * t2274 + 0.32163958997385070134e2 * t2279 * t2282 + t2191 - t2194 + t2201 - t2237 - t2245 - 0.19751673498613801407e-1 * t2289 + 0.11696447245269292414e1 * t2291 * t891 - 0.11696447245269292414e1 * t2296 * t2298 + 0.5848223622634646207e0 * t882 * t2313 + 0.17315859105681463759e2 * t2318 * t2321;
    (t2324,)
}
