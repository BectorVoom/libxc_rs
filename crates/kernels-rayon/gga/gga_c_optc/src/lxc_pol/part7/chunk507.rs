//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 507/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk507(t2472: f64, t256: f64, t2473: f64, t2476: f64, t2377: f64, t2380: f64, t2412: f64, t2421: f64, t2428: f64, t2451: f64, t2485: f64, t2488: f64, t2493: f64, t2495: f64, t2513: f64, t2518: f64, t252: f64, t2521: f64, t2525: f64, t2530: f64, t2531: f64, t2534: f64, t810: f64, t819: f64, t829: f64, t838: f64) -> (f64, f64, f64) {
    let t2537 = t256 * t2472;
    let t2538 = t2473 * t2476;
    let t2541 = -0.3109e-1_f64 * t2485 * t252 + 2.0_f64 * t2488 * t819 - 2.0_f64 * t2493 * t2495 + 1.0_f64 * t810 * t2513 + 0.32164683177870697974e2_f64 * t2518 * t2521 + t2451 - t2380 + t2377 - t2412 - t2421 - 0.19751789702565206229e-1_f64 * t2428 + 0.11696446794910408142e1_f64 * t2525 * t838 - 0.11696446794910408142e1_f64 * t2530 * t2531 + 0.58482233974552040708e0_f64 * t829 * t2534 + 0.17315755899375863299e2_f64 * t2537 * t2538;
    (t2537, t2538, t2541)
}
