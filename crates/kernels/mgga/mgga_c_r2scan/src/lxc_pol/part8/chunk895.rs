//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 895/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk895<F: Float>(t2294: F, t2599: F, t2598: F, t1632: F, t2625: F, t551: F, t2196: F, t2155: F, t7407: F, t2609: F, t6395: F, t113: F, t7433: F, t2148: F, t2147: F, t7503: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8044 = t2294 * t2599;
    let t8046 = 0.46230515946956099004e0 * t2598 * t8044;
    let t8054 = t551 * t1632 * t2625;
    let t8056 = 0.27738309568173659402e1 * t2196 * t8054;
    let t8062 = t2155 * t7407;
    let t8065 = 0.11643651550782197811e-1 * t6395 * t2609;
    let t8066 = t7433 * t113;
    let t8067 = t2148 * t8066;
    let t8069 = 0.11643651550782197811e-1 * t2147 * t8067;
    let t8070 = t7503 * t113;
    (t8044, t8046, t8054, t8056, t8062, t8065, t8066, t8067, t8069, t8070)
}
