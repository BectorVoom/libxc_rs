//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1078/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1078<F: Float>(t17095: F, t618: F, t1780: F, t5296: F, t1776: F, t187: F, t5417: F, t1675: F, t1816: F, t568: F, t5162: F, t5165: F) -> (F, F, F, F, F, F, F) {
    let t17096 = t17095 * t618;
    let t17098 = t5296 * t1780;
    let t17100 = t5296 * t1776;
    let t17121 = F::new(1.0) / t5417 / t187;
    let t17244 = t1675 * t1675;
    let t17245 = F::new(1.0) / t17244;
    let t17258 = t568 * t1816;
    let t17272 = t5162 * t5165;
    (t17096, t17098, t17100, t17121, t17245, t17258, t17272)
}
