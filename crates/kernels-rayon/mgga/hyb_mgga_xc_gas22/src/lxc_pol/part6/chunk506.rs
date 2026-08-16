//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 506/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk506(t2289: f64, t2291: f64, t848: f64, t2306: f64, t839: f64, t2311: f64, t2314: f64, t2182: f64, t2185: f64, t2192: f64, t2231: f64, t2239: f64, t2283: f64, t2318: f64, t2322: f64, t260: f64, t856: f64, t858: f64) -> (f64, f64, f64, f64) {
    let t2326 = t2289 * t2291 * t848;
    let t2330 = t839 * t2306 * t848;
    let t2333 = t2311 * t2291;
    let t2334 = t2333 * t2314;
    let t2337 = -t2182 + t2185 - t2192 + t2231 + t2239 + t260 * t2318 + 0.19751673498613801407e-1_f64 * t260 * t2283 - 0.11696447245269292414e1_f64 * t2322 * t858 + 0.11696447245269292414e1_f64 * t856 * t2326 - 0.5848223622634646207e0_f64 * t856 * t2330 - 0.17315859105681463759e2_f64 * t856 * t2334;
    (t2326, t2330, t2334, t2337)
}
