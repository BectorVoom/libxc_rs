//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1136/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1136(t10: f64, t4475: f64, t1107: f64, t1052: f64, t4482: f64, t1057: f64, t7526: f64, t7528: f64, t7535: f64, t7546: f64, t7549: f64, t7551: f64, t7556: f64, t9392: f64, t9397: f64, t9398: f64, t9399: f64, t9402: f64, t9406: f64, t9407: f64) -> (f64, f64) {
    let t11245 = t4475 * t10;
    let t11246 = t11245 * t1107;
    let t11248 = t1052 * t4482;
    let t11250 = t1057 * t4482;
    let t11256 = 0.23392894490538584828e1_f64 * t9392 - 8.0_f64 * t7526 - 8.0_f64 * t7528 + t7535 + t9397 - t9398 - t7546 - 0.18311447306006545054e-3_f64 * t11246 - t9399 + 4.0_f64 * t11248 - 4.0_f64 * t11250 - 0.17315859105681463759e2_f64 * t7549 - 0.5848223622634646207e0_f64 * t7551 - t7556 - 0.34631718211362927517e2_f64 * t9402 - t9406 - 0.11696447245269292414e1_f64 * t9407;
    (t11245, t11256)
}
