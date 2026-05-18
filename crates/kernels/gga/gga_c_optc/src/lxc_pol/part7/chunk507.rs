//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 507/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk507<F: Float>(t2472: F, t256: F, t2473: F, t2476: F, t2377: F, t2380: F, t2412: F, t2421: F, t2428: F, t2451: F, t2485: F, t2488: F, t2493: F, t2495: F, t2513: F, t2518: F, t252: F, t2521: F, t2525: F, t2530: F, t2531: F, t2534: F, t810: F, t819: F, t829: F, t838: F) -> (F, F, F) {
    let t2537 = t256 * t2472;
    let t2538 = t2473 * t2476;
    let t2541 = -F::new(0.3109e-1) * t2485 * t252 + F::new(2.0) * t2488 * t819 - F::new(2.0) * t2493 * t2495 + F::new(1.0) * t810 * t2513 + F::new(0.32164683177870697974e2) * t2518 * t2521 + t2451 - t2380 + t2377 - t2412 - t2421 - F::new(0.19751789702565206229e-1) * t2428 + F::new(0.11696446794910408142e1) * t2525 * t838 - F::new(0.11696446794910408142e1) * t2530 * t2531 + F::new(0.58482233974552040708e0) * t829 * t2534 + F::new(0.17315755899375863299e2) * t2537 * t2538;
    (t2537, t2538, t2541)
}
