//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 783/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk783<F: Float>(t2890: F, t68: F, t2887: F, t2739: F, t779: F, t297: F, t46: F, t768: F, t2942: F, t2883: F, t735: F, t1066: F, t154: F, t5688: F) -> (F, F, F, F, F) {
    let t7589 = t68 * t2890;
    let t7591 = t2887 * t7589 / F::cast_from(72.0_f64);
    let t7592 = t779 * t2739;
    let t7606 = t768 * t297 * t46;
    let t7607 = t2942 * t7606;
    let t7617 = t735 * t2883 / F::cast_from(54.0_f64);
    let t7620 = t154 * t5688 * t1066;
    (t7591, t7592, t7607, t7617, t7620)
}
