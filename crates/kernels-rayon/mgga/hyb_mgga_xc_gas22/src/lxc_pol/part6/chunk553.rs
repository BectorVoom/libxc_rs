//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 553/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk553(t1007: f64, t2472: f64, t2475: f64, t2482: f64, t2518: f64, t2526: f64, t2530: f64, t2533: f64, t2538: f64, t2540: f64, t2555: f64, t2560: f64, t2563: f64, t2570: f64, t2572: f64, t2577: f64, t2579: f64, t2594: f64, t2599: f64, t2602: f64, t374: f64, t979: f64, t988: f64, t998: f64) -> f64 {
    let t2605 = -0.310907e-1_f64 * t2530 * t374 + 2.0_f64 * t2533 * t988 - 2.0_f64 * t2538 * t2540 + 1.0_f64 * t979 * t2555 + 0.32163958997385070134e2_f64 * t2560 * t2563 + t2472 - t2475 + t2482 - t2518 - t2526 - 0.19751673498613801407e-1_f64 * t2570 + 0.11696447245269292414e1_f64 * t2572 * t1007 - 0.11696447245269292414e1_f64 * t2577 * t2579 + 0.5848223622634646207e0_f64 * t998 * t2594 + 0.17315859105681463759e2_f64 * t2599 * t2602;
    t2605
}
