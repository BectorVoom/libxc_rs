//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 870/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk870(t2273: f64, t2281: f64, t870: f64, t6087: f64, t6090: f64, t6093: f64, t6108: f64, t2252: f64, t2257: f64, t2274: f64, t2279: f64, t2282: f64, t2291: f64, t2296: f64, t2313: f64, t2318: f64, t2321: f64, t365: f64, t6132: f64, t6300: f64, t6303: f64, t6308: f64, t6313: f64, t6314: f64, t6319: f64, t6322: f64, t6323: f64, t6324: f64, t6329: f64, t6333: f64, t6334: f64, t6338: f64, t6341: f64, t872: f64) -> (f64, f64, f64, f64) {
    let t6345 = t2273 * t2281 * t870;
    let t6348 = 0.53272592592592592592e-1_f64 * t6087;
    let t6352 = -t6348 + 0.68493333333333333332e-1_f64 * t6090 - 0.51369999999999999999e-1_f64 * t6093 + 0.5137e-1_f64 * t6108;
    let t6356 = 0.17544670867903938621e1_f64 * t2291 * t2313 + 0.51947577317044391276e2_f64 * t6300 * t2321 + 3.0_f64 * t6303 * t872 + 3.0_f64 * t2252 * t2274 + 0.96491876992155210402e2_f64 * t6308 * t2282 - 0.19298375398431042081e3_f64 * t6313 * t6314 + t6319 - t6322 - 0.10389515463408878255e3_f64 * t6323 * t6324 + t6329 - t6333 - 0.35089341735807877242e1_f64 * t2296 * t6334 + 0.51947577317044391277e2_f64 * t2318 * t6338 - 6.0_f64 * t2257 * t6341 + 0.96491876992155210402e2_f64 * t2279 * t6345 - 0.310907e-1_f64 * t6352 * t365 - 0.19751673498613801407e-1_f64 * t6132;
    (t6345, t6348, t6352, t6356)
}
