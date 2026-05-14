//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1213/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1213<F: Float>(t237: F, t6323: F, t1208: F, t2295: F, t3113: F, t1201: F, t6121: F, t1196: F, t6288: F, t2279: F, t6282: F, t3102: F, t6290: F, t6313: F, t3135: F, t6233: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22503 = t237 * t6323;
    let t22561 = t6323 * t1208;
    let t22564 = t3113 * t2295;
    let t22567 = t1201 * t6121;
    let t22570 = t6288 * t1196;
    let t22575 = t2279 * t1196;
    let t22578 = t6282 * t1208;
    let t22627 = t3102 * t6290;
    let t22639 = t6313 * t1196;
    let t22662 = t3135 * t6233;
    (t22503, t22561, t22564, t22567, t22570, t22575, t22578, t22627, t22639, t22662)
}
