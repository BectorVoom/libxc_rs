//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 682/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk682<F: Float>(t3422: F, t847: F, t1359: F, t1371: F, t2246: F, t2251: F, t2273: F, t2285: F, t2290: F, t2312: F, t271: F, t3315: F, t3318: F, t3320: F, t3323: F, t3355: F, t3359: F, t3363: F, t3366: F, t3371: F, t3386: F, t3390: F, t3397: F, t3399: F, t3404: F, t3419: F, t821: F, t830: F, t840: F, t849: F) -> (F, F) {
    let t3423 = t3422 * t847;
    let t3426 = -F::cast_from(0.310907e-1_f64) * t3363 * t271 + F::cast_from(1.0_f64) * t3366 * t830 + F::cast_from(1.0_f64) * t2246 * t1359 - F::cast_from(2.0_f64) * t2251 * t3371 + F::cast_from(1.0_f64) * t821 * t3386 + F::cast_from(0.32163958997385070134e2_f64) * t2273 * t3390 + t3315 - t3318 - t3320 + t3323 - t3355 - t3359 - F::cast_from(0.19751673498613801407e-1_f64) * t3397 + F::cast_from(0.5848223622634646207e0_f64) * t3399 * t849 + F::cast_from(0.5848223622634646207e0_f64) * t2285 * t1371 - F::cast_from(0.11696447245269292414e1_f64) * t2290 * t3404 + F::cast_from(0.5848223622634646207e0_f64) * t840 * t3419 + F::cast_from(0.17315859105681463759e2_f64) * t2312 * t3423;
    (t3423, t3426)
}
