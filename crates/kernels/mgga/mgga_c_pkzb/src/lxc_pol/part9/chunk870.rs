//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 870/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk870<F: Float>(t2273: F, t2281: F, t870: F, t6087: F, t6090: F, t6093: F, t6108: F, t2252: F, t2257: F, t2274: F, t2279: F, t2282: F, t2291: F, t2296: F, t2313: F, t2318: F, t2321: F, t365: F, t6132: F, t6300: F, t6303: F, t6308: F, t6313: F, t6314: F, t6319: F, t6322: F, t6323: F, t6324: F, t6329: F, t6333: F, t6334: F, t6338: F, t6341: F, t872: F) -> (F, F, F, F) {
    let t6345 = t2273 * t2281 * t870;
    let t6348 = F::new(0.53272592592592592592e-1) * t6087;
    let t6352 = -t6348 + F::new(0.68493333333333333332e-1) * t6090 - F::new(0.51369999999999999999e-1) * t6093 + F::new(0.5137e-1) * t6108;
    let t6356 = F::new(0.17544670867903938621e1) * t2291 * t2313 + F::new(0.51947577317044391276e2) * t6300 * t2321 + F::new(3.0) * t6303 * t872 + F::new(3.0) * t2252 * t2274 + F::new(0.96491876992155210402e2) * t6308 * t2282 - F::new(0.19298375398431042081e3) * t6313 * t6314 + t6319 - t6322 - F::new(0.10389515463408878255e3) * t6323 * t6324 + t6329 - t6333 - F::new(0.35089341735807877242e1) * t2296 * t6334 + F::new(0.51947577317044391277e2) * t2318 * t6338 - F::new(6.0) * t2257 * t6341 + F::new(0.96491876992155210402e2) * t2279 * t6345 - F::new(0.310907e-1) * t6352 * t365 - F::new(0.19751673498613801407e-1) * t6132;
    (t6345, t6348, t6352, t6356)
}
