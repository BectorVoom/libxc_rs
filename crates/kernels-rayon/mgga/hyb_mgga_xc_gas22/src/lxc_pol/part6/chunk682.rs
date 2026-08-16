//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 682/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk682(t3422: f64, t847: f64, t1359: f64, t1371: f64, t2246: f64, t2251: f64, t2273: f64, t2285: f64, t2290: f64, t2312: f64, t271: f64, t3315: f64, t3318: f64, t3320: f64, t3323: f64, t3355: f64, t3359: f64, t3363: f64, t3366: f64, t3371: f64, t3386: f64, t3390: f64, t3397: f64, t3399: f64, t3404: f64, t3419: f64, t821: f64, t830: f64, t840: f64, t849: f64) -> (f64, f64) {
    let t3423 = t3422 * t847;
    let t3426 = -0.310907e-1_f64 * t3363 * t271 + 1.0_f64 * t3366 * t830 + 1.0_f64 * t2246 * t1359 - 2.0_f64 * t2251 * t3371 + 1.0_f64 * t821 * t3386 + 0.32163958997385070134e2_f64 * t2273 * t3390 + t3315 - t3318 - t3320 + t3323 - t3355 - t3359 - 0.19751673498613801407e-1_f64 * t3397 + 0.5848223622634646207e0_f64 * t3399 * t849 + 0.5848223622634646207e0_f64 * t2285 * t1371 - 0.11696447245269292414e1_f64 * t2290 * t3404 + 0.5848223622634646207e0_f64 * t840 * t3419 + 0.17315859105681463759e2_f64 * t2312 * t3423;
    (t3423, t3426)
}
