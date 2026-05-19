//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 506/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk506<F: Float>(t2289: F, t2291: F, t848: F, t2306: F, t839: F, t2311: F, t2314: F, t2182: F, t2185: F, t2192: F, t2231: F, t2239: F, t2283: F, t2318: F, t2322: F, t260: F, t856: F, t858: F) -> (F, F, F, F) {
    let t2326 = t2289 * t2291 * t848;
    let t2330 = t839 * t2306 * t848;
    let t2333 = t2311 * t2291;
    let t2334 = t2333 * t2314;
    let t2337 = -t2182 + t2185 - t2192 + t2231 + t2239 + t260 * t2318 + F::cast_from(0.19751673498613801407e-1_f64) * t260 * t2283 - F::cast_from(0.11696447245269292414e1_f64) * t2322 * t858 + F::cast_from(0.11696447245269292414e1_f64) * t856 * t2326 - F::cast_from(0.5848223622634646207e0_f64) * t856 * t2330 - F::cast_from(0.17315859105681463759e2_f64) * t856 * t2334;
    (t2326, t2330, t2334, t2337)
}
