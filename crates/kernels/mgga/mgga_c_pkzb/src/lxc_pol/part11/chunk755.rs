//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 755/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk755<F: Float>(t237: F, t2826: F, t1125: F, t5939: F, t757: F, t2096: F, t2908: F, t2886: F, t434: F, t2890: F, t68: F, t2887: F, t2739: F, t779: F, t297: F, t46: F, t768: F) -> (F, F, F, F, F, F, F, F) {
    let t7560 = t237 * t2826;
    let t7581 = t5939 * t1125;
    let t7582 = t757 * t7581;
    let t7585 = 0.15244095330869239812e-2 * t2096 * t2908;
    let t7586 = t434 * t2886;
    let t7589 = t68 * t2890;
    let t7591 = t2887 * t7589 / 72.0;
    let t7592 = t779 * t2739;
    let t7606 = t768 * t297 * t46;
    (t7560, t7581, t7582, t7585, t7586, t7591, t7592, t7606)
}
