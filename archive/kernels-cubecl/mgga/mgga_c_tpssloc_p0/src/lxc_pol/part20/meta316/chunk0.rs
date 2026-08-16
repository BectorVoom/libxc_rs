//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1577/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1577<F: Float>(t11516: F, t9288: F, t3440: F, t3441: F, t1177: F, t1178: F, t9258: F, t1176: F, t698: F) -> (F, F, F, F, F, F, F) {
    let t11517 = t11516 * t9288;
    let t11518 = t3440 * t11517;
    let t11521 = t3441 * t9288;
    let t11522 = t1177 * t11521;
    let t11525 = t1178 * t9258;
    let t11526 = t1177 * t11525;
    let t11529 = t698 * t1176;
    (t11517, t11518, t11521, t11522, t11525, t11526, t11529)
}
