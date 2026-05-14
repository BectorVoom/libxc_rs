//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 979/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk979<F: Float>(t2572: F, t7378: F, t360: F, t2195: F, t2666: F, t6343: F, t938: F, t551: F, t549: F, t1632: F, t2719: F, t2169: F, t2731: F, t2236: F, t2727: F, t2219: F, t2670: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7379 = t2572 * t7378;
    let t7380 = t360 * t7379;
    let t7383 = t2195 * t2666;
    let t7386 = t6343 * t938;
    let t7387 = t551 * t7386;
    let t7388 = t549 * t7387;
    let t7390 = t1632 * t2719;
    let t7391 = t551 * t7390;
    let t7393 = 0.23115257973478049502e0 * t549 * t7391;
    let t7395 = 0.69345773920434148506e0 * t2169 * t2731;
    let t7397 = 0.23115257973478049502e0 * t2236 * t2727;
    let t7399 = 0.69345773920434148506e0 * t2670 * t2219;
    (t7379, t7380, t7383, t7387, t7388, t7391, t7393, t7395, t7397, t7399)
}
