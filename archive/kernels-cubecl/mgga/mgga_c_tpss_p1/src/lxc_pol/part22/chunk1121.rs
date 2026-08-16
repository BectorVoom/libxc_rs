//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1121/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1121<F: Float>(t12378: F, t581: F, t12377: F, t3068: F, t1557: F, t672: F, t1098: F, t1561: F, t3054: F, t1014: F, t1113: F, t12372: F, t4046: F) -> (F, F, F, F) {
    let t12379 = t12378 * t581;
    let t12380 = t12377 * t12379;
    let t12381 = t3068 * t12380;
    let t12384 = t672 * t1557;
    let t12385 = t1098 * t12384;
    let t12387 = t1561 * t3054;
    let t12389 = t1113 * t1014 * t581;
    let t12390 = t12387 * t12389;
    let t12391 = t3068 * t12390;
    let t12394 = t4046 * t12372;
    (t12381, t12385, t12391, t12394)
}
