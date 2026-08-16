//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 504/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk504(t2182: f64, t2185: f64, t2192: f64, t2231: f64, t2239: f64, t2243: f64, t2246: f64, t2251: f64, t2253: f64, t2268: f64, t2273: f64, t2276: f64, t2283: f64, t2285: f64, t2290: f64, t2292: f64, t2307: f64, t2312: f64, t2315: f64, t271: f64, t821: f64, t830: f64, t840: f64, t849: f64) -> f64 {
    let t2318 = -0.310907e-1_f64 * t2243 * t271 + 2.0_f64 * t2246 * t830 - 2.0_f64 * t2251 * t2253 + 1.0_f64 * t821 * t2268 + 0.32163958997385070134e2_f64 * t2273 * t2276 + t2182 - t2185 + t2192 - t2231 - t2239 - 0.19751673498613801407e-1_f64 * t2283 + 0.11696447245269292414e1_f64 * t2285 * t849 - 0.11696447245269292414e1_f64 * t2290 * t2292 + 0.5848223622634646207e0_f64 * t840 * t2307 + 0.17315859105681463759e2_f64 * t2312 * t2315;
    t2318
}
