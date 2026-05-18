//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1308/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1308<F: Float>(t11369: F, t2029: F, t11426: F, t2099: F, t6525: F, t10047: F, t10051: F, t10056: F, t10072: F, t10103: F, t10192: F, t11519: F, t1227: F, t22461: F, t2380: F, t2381: F, t2382: F, t28195: F, t3185: F, t3187: F, t3199: F, t3206: F, t3207: F, t3214: F, t3730: F, t3757: F, t406: F, t6366: F, t8319: F, t921: F, t9795: F) -> F {
    let t31700 = t11369 * t2029;
    let t31734 = t6525 * t2099 * t11426;
    let t31736 = -F::new(0.34299214494455789577e-2) * t3214 * t10192 + F::new(0.68598428988911579154e-2) * t10047 * t10072 - F::new(0.34299214494455789577e-2) * t28195 * t10103 + F::new(0.42874018118069736972e-3) * t3185 * t406 * t31700 * t3187 - F::new(0.21437009059034868486e-3) * t3206 * t406 * t31700 * t3207 - F::new(0.42874018118069736972e-3) * t2380 * t2381 * t11519 * t2382 + F::new(0.13719685797782315831e-1) * t8319 * t10056 + F::new(0.13719685797782315831e-1) * t8319 * t10051 - F::new(0.12862205435420921092e-2) * t2380 * t2381 * t9795 * t1227 * t921 - F::new(0.12862205435420921092e-2) * t2380 * t2381 * t3730 * t3199 * t921 + F::new(0.38586616306262763275e-2) * t2380 * t6366 * t3757 * t3199 * t921 - F::new(5.0) / F::new(162.0) * t22461 - F::new(0.85748036236139473947e-3) * t31734;
    t31736
}
