//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 580/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk580(t2411: f64, t66: f64, t179: f64, t2226: f64, t2185: f64, t932: f64, t2346: f64, t2350: f64, t2354: f64, t2358: f64, t2367: f64, t2373: f64, t2377: f64, t2380: f64, t2384: f64, t2390: f64, t2395: f64, t2398: f64, t2404: f64, t2408: f64, t385: f64, t404: f64, t918: f64) -> (f64, f64, f64) {
    let t2412 = t66 * t2411;
    let t2414 = t179 * t2412 * t2226;
    let t2418 = t179 * t932 * t2185;
    let t2421 = -t2346 - t2350 / 144.0_f64 + t385 * t2354 / 48.0_f64 - t385 * t2358 / 96.0_f64 + 0.42874018118069736972e-3_f64 * t2367 * t2373 + 0.28582678745379824648e-3_f64 * t2377 - 0.85748036236139473944e-3_f64 * t2380 * t2384 + 0.21437009059034868486e-3_f64 * t918 * t2390 - 0.21437009059034868486e-3_f64 * t2395 * t2398 - t2404 - 0.57165357490759649296e-3_f64 * t2408 + 0.12862205435420921092e-2_f64 * t404 * t2414 - 0.42874018118069736972e-3_f64 * t404 * t2418;
    (t2414, t2418, t2421)
}
