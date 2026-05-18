//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 580/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk580<F: Float>(t2411: F, t66: F, t179: F, t2226: F, t2185: F, t932: F, t2346: F, t2350: F, t2354: F, t2358: F, t2367: F, t2373: F, t2377: F, t2380: F, t2384: F, t2390: F, t2395: F, t2398: F, t2404: F, t2408: F, t385: F, t404: F, t918: F) -> (F, F, F) {
    let t2412 = t66 * t2411;
    let t2414 = t179 * t2412 * t2226;
    let t2418 = t179 * t932 * t2185;
    let t2421 = -t2346 - t2350 / F::new(144.0) + t385 * t2354 / F::new(48.0) - t385 * t2358 / F::new(96.0) + F::new(0.42874018118069736972e-3) * t2367 * t2373 + F::new(0.28582678745379824648e-3) * t2377 - F::new(0.85748036236139473944e-3) * t2380 * t2384 + F::new(0.21437009059034868486e-3) * t918 * t2390 - F::new(0.21437009059034868486e-3) * t2395 * t2398 - t2404 - F::new(0.57165357490759649296e-3) * t2408 + F::new(0.12862205435420921092e-2) * t404 * t2414 - F::new(0.42874018118069736972e-3) * t404 * t2418;
    (t2414, t2418, t2421)
}
