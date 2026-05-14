//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 807/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk807<F: Float>(t8768: F, t8793: F, t9120: F, t9125: F, t45: F, t3715: F, t645: F, t1116: F, t7560: F, t2860: F, t2870: F, t1987: F, t3618: F, t3528: F, t5511: F, t667: F) -> (F, F, F, F, F, F, F, F) {
    let t9127 = t8768 + t8793 + t9120 + t9125;
    let t9128 = t45 * t9127;
    let t9129 = t645 * t3715;
    let t9132 = 0.11696447245269292414e1 * t7560 * t1116;
    let t9134 = 0.11696447245269292414e1 * t2860 * t2870;
    let t9136 = 0.11696447245269292414e1 * t1987 * t3618;
    let t9137 = t5511 * t3528;
    let t9138 = t9137 * t667;
    (t9127, t9128, t9129, t9132, t9134, t9136, t9137, t9138)
}
