//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1010/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1010<F: Float>(t1249: F, t6367: F, t6366: F, t2029: F, t3199: F, t3187: F, t406: F, t2376: F, t3214: F, t1238: F, t2407: F, t3195: F, t6475: F, t2380: F, t1167: F, t179: F, t6380: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8376 = t1249 * t6367;
    let t8377 = t6366 * t8376;
    let t8380 = t3199 * t2029;
    let t8381 = t8380 * t3187;
    let t8382 = t406 * t8381;
    let t8386 = 0.15244095330869239812e-2 * t3214 * t2376;
    let t8389 = 0.30488190661738479624e-2 * t1238 * t2407;
    let t8392 = t6475 * t3195;
    let t8394 = 0.57165357490759649296e-3 * t2380 * t8392;
    let t8397 = t179 * t6380 * t1167;
    (t8376, t8377, t8380, t8381, t8382, t8386, t8389, t8392, t8394, t8397)
}
