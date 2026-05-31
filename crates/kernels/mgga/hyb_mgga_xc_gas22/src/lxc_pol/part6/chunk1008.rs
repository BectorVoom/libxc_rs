//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1008/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1008<F: Float>(t2649: F, t3616: F, t7537: F, t7540: F, t7547: F, t2640: F, t3636: F, t483: F, t1112: F, t2676: F, t7522: F, t7523: F, t7528: F, t7530: F, t7532: F, t7535: F, t7546: F, t7549: F, t7551: F, t7556: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9392 = t3616 * t2649;
    let t9397 = F::cast_from(48.0_f64) * t7537;
    let t9398 = F::cast_from(80.0_f64) * t7540;
    let t9399 = F::cast_from(4.0_f64) * t7547;
    let t9402 = t3616 * t2640;
    let t9404 = t3636 * t483;
    let t9406 = F::cast_from(0.11696447245269292414e1_f64) * t9404 * t1112;
    let t9407 = t3616 * t2676;
    let t9409 = -t7522 + F::cast_from(0.4883052614935078681e-3_f64) * t7523 + F::cast_from(0.11696447245269292414e1_f64) * t9392 - F::cast_from(16.0_f64) * t7528 - F::cast_from(4.0_f64) * t7530 - F::cast_from(4.0_f64) * t7532 - t7535 + t9397 + t9398 - t7546 + t9399 - F::cast_from(0.34631718211362927518e2_f64) * t7549 - F::cast_from(0.11696447245269292414e1_f64) * t7551 - t7556 - F::cast_from(0.17315859105681463759e2_f64) * t9402 - t9406 - F::cast_from(0.5848223622634646207e0_f64) * t9407;
    (t9392, t9397, t9398, t9399, t9402, t9404, t9406, t9407, t9409)
}
