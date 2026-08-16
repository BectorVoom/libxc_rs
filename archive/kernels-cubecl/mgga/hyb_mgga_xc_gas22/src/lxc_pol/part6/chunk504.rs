//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 504/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk504<F: Float>(t2182: F, t2185: F, t2192: F, t2231: F, t2239: F, t2243: F, t2246: F, t2251: F, t2253: F, t2268: F, t2273: F, t2276: F, t2283: F, t2285: F, t2290: F, t2292: F, t2307: F, t2312: F, t2315: F, t271: F, t821: F, t830: F, t840: F, t849: F) -> F {
    let t2318 = -F::cast_from(0.310907e-1_f64) * t2243 * t271 + F::cast_from(2.0_f64) * t2246 * t830 - F::cast_from(2.0_f64) * t2251 * t2253 + F::cast_from(1.0_f64) * t821 * t2268 + F::cast_from(0.32163958997385070134e2_f64) * t2273 * t2276 + t2182 - t2185 + t2192 - t2231 - t2239 - F::cast_from(0.19751673498613801407e-1_f64) * t2283 + F::cast_from(0.11696447245269292414e1_f64) * t2285 * t849 - F::cast_from(0.11696447245269292414e1_f64) * t2290 * t2292 + F::cast_from(0.5848223622634646207e0_f64) * t840 * t2307 + F::cast_from(0.17315859105681463759e2_f64) * t2312 * t2315;
    t2318
}
