//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 553/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk553<F: Float>(t1007: F, t2472: F, t2475: F, t2482: F, t2518: F, t2526: F, t2530: F, t2533: F, t2538: F, t2540: F, t2555: F, t2560: F, t2563: F, t2570: F, t2572: F, t2577: F, t2579: F, t2594: F, t2599: F, t2602: F, t374: F, t979: F, t988: F, t998: F) -> F {
    let t2605 = -F::new(0.310907e-1) * t2530 * t374 + F::new(2.0) * t2533 * t988 - F::new(2.0) * t2538 * t2540 + F::new(1.0) * t979 * t2555 + F::cast_from(0.32163958997385070134e2_f64) * t2560 * t2563 + t2472 - t2475 + t2482 - t2518 - t2526 - F::cast_from(0.19751673498613801407e-1_f64) * t2570 + F::cast_from(0.11696447245269292414e1_f64) * t2572 * t1007 - F::cast_from(0.11696447245269292414e1_f64) * t2577 * t2579 + F::cast_from(0.5848223622634646207e0_f64) * t998 * t2594 + F::cast_from(0.17315859105681463759e2_f64) * t2599 * t2602;
    t2605
}
