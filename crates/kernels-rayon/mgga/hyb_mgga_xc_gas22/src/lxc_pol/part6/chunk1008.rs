//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1008/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1008(t2649: f64, t3616: f64, t7537: f64, t7540: f64, t7547: f64, t2640: f64, t3636: f64, t483: f64, t1112: f64, t2676: f64, t7522: f64, t7523: f64, t7528: f64, t7530: f64, t7532: f64, t7535: f64, t7546: f64, t7549: f64, t7551: f64, t7556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9392 = t3616 * t2649;
    let t9397 = 48.0_f64 * t7537;
    let t9398 = 80.0_f64 * t7540;
    let t9399 = 4.0_f64 * t7547;
    let t9402 = t3616 * t2640;
    let t9404 = t3636 * t483;
    let t9406 = 0.11696447245269292414e1_f64 * t9404 * t1112;
    let t9407 = t3616 * t2676;
    let t9409 = -t7522 + 0.4883052614935078681e-3_f64 * t7523 + 0.11696447245269292414e1_f64 * t9392 - 16.0_f64 * t7528 - 4.0_f64 * t7530 - 4.0_f64 * t7532 - t7535 + t9397 + t9398 - t7546 + t9399 - 0.34631718211362927518e2_f64 * t7549 - 0.11696447245269292414e1_f64 * t7551 - t7556 - 0.17315859105681463759e2_f64 * t9402 - t9406 - 0.5848223622634646207e0_f64 * t9407;
    (t9392, t9397, t9398, t9399, t9402, t9404, t9406, t9407, t9409)
}
