//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 555/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk555(t1006: f64, t2576: f64, t2578: f64, t2593: f64, t997: f64, t2598: f64, t2601: f64, t1014: f64, t1016: f64, t2472: f64, t2475: f64, t2482: f64, t2518: f64, t2526: f64, t2570: f64, t260: f64, t2605: f64, t2609: f64) -> (f64, f64, f64, f64) {
    let t2613 = t2576 * t2578 * t1006;
    let t2617 = t997 * t2593 * t1006;
    let t2620 = t2598 * t2578;
    let t2621 = t2620 * t2601;
    let t2624 = -t2472 + t2475 - t2482 + t2518 + t2526 + t260 * t2605 + 0.19751673498613801407e-1_f64 * t260 * t2570 - 0.11696447245269292414e1_f64 * t2609 * t1016 + 0.11696447245269292414e1_f64 * t1014 * t2613 - 0.5848223622634646207e0_f64 * t1014 * t2617 - 0.17315859105681463759e2_f64 * t1014 * t2621;
    (t2613, t2617, t2621, t2624)
}
