//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1136/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1136<F: Float>(t10: F, t4475: F, t1107: F, t1052: F, t4482: F, t1057: F, t7526: F, t7528: F, t7535: F, t7546: F, t7549: F, t7551: F, t7556: F, t9392: F, t9397: F, t9398: F, t9399: F, t9402: F, t9406: F, t9407: F) -> (F, F) {
    let t11245 = t4475 * t10;
    let t11246 = t11245 * t1107;
    let t11248 = t1052 * t4482;
    let t11250 = t1057 * t4482;
    let t11256 = F::new(0.23392894490538584828e1) * t9392 - F::new(8.0) * t7526 - F::new(8.0) * t7528 + t7535 + t9397 - t9398 - t7546 - F::new(0.18311447306006545054e-3) * t11246 - t9399 + F::new(4.0) * t11248 - F::new(4.0) * t11250 - F::new(0.17315859105681463759e2) * t7549 - F::new(0.5848223622634646207e0) * t7551 - t7556 - F::new(0.34631718211362927517e2) * t9402 - t9406 - F::new(0.11696447245269292414e1) * t9407;
    (t11245, t11256)
}
