//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1288/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1288<F: Float>(t29279: F, t6375: F, t7623: F, t1568: F, t29283: F, t2530: F, t921: F, t538: F, t2687: F, t27217: F, t7624: F, t20594: F, t2691: F, t8081: F, t25359: F, t7619: F) -> (F, F, F, F, F, F, F) {
    let t30315 = t7623 * t6375 * t29279;
    let t30318 = t7623 * t1568 * t29283;
    let t30320 = t921 * t2530;
    let t30322 = t7623 * t538 * t30320;
    let t30333 = t27217 * t2687 * t7624;
    let t30339 = t20594 * t2691 * t8081;
    let t30342 = t25359 * t2691 * t7619;
    (t30315, t30318, t30320, t30322, t30333, t30339, t30342)
}
