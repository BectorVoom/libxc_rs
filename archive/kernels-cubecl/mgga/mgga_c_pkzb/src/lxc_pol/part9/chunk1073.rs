//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1073/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1073<F: Float>(t123: F, t1697: F, t475: F, t550: F, t46: F, t5119: F, t552: F, t1667: F, t5327: F, t1505: F, t1618: F, t555: F) -> (F, F, F, F) {
    let t16915 = F::cast_from(0.18989649058080861537e-2_f64) * t550 * t475 * t1697 * t123;
    let t16917 = t5119 * t46 * t552;
    let t16919 = t5327 * t1667;
    let t16923 = F::cast_from(0.21053605041484726346e2_f64) * t555 * t1505 * t1618;
    (t16915, t16917, t16919, t16923)
}
