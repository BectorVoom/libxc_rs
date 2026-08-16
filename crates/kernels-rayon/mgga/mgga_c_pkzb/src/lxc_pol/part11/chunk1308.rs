//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1308/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1308(t11369: f64, t2029: f64, t11426: f64, t2099: f64, t6525: f64, t10047: f64, t10051: f64, t10056: f64, t10072: f64, t10103: f64, t10192: f64, t11519: f64, t1227: f64, t22461: f64, t2380: f64, t2381: f64, t2382: f64, t28195: f64, t3185: f64, t3187: f64, t3199: f64, t3206: f64, t3207: f64, t3214: f64, t3730: f64, t3757: f64, t406: f64, t6366: f64, t8319: f64, t921: f64, t9795: f64) -> f64 {
    let t31700 = t11369 * t2029;
    let t31734 = t6525 * t2099 * t11426;
    let t31736 = -0.34299214494455789577e-2_f64 * t3214 * t10192 + 0.68598428988911579154e-2_f64 * t10047 * t10072 - 0.34299214494455789577e-2_f64 * t28195 * t10103 + 0.42874018118069736972e-3_f64 * t3185 * t406 * t31700 * t3187 - 0.21437009059034868486e-3_f64 * t3206 * t406 * t31700 * t3207 - 0.42874018118069736972e-3_f64 * t2380 * t2381 * t11519 * t2382 + 0.13719685797782315831e-1_f64 * t8319 * t10056 + 0.13719685797782315831e-1_f64 * t8319 * t10051 - 0.12862205435420921092e-2_f64 * t2380 * t2381 * t9795 * t1227 * t921 - 0.12862205435420921092e-2_f64 * t2380 * t2381 * t3730 * t3199 * t921 + 0.38586616306262763275e-2_f64 * t2380 * t6366 * t3757 * t3199 * t921 - 5.0_f64 / 162.0_f64 * t22461 - 0.85748036236139473947e-3_f64 * t31734;
    t31736
}
