//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 564/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk564(t2191: f64, t2194: f64, t2201: f64, t2237: f64, t2245: f64, t2249: f64, t2252: f64, t2257: f64, t2259: f64, t2274: f64, t2279: f64, t2282: f64, t2289: f64, t2291: f64, t2296: f64, t2298: f64, t2313: f64, t2318: f64, t2321: f64, t365: f64, t863: f64, t872: f64, t882: f64, t891: f64) -> f64 {
    let t2324 = -0.310907e-1_f64 * t2249 * t365 + 2.0_f64 * t2252 * t872 - 2.0_f64 * t2257 * t2259 + 1.0_f64 * t863 * t2274 + 0.32163958997385070134e2_f64 * t2279 * t2282 + t2191 - t2194 + t2201 - t2237 - t2245 - 0.19751673498613801407e-1_f64 * t2289 + 0.11696447245269292414e1_f64 * t2291 * t891 - 0.11696447245269292414e1_f64 * t2296 * t2298 + 0.5848223622634646207e0_f64 * t882 * t2313 + 0.17315859105681463759e2_f64 * t2318 * t2321;
    t2324
}
