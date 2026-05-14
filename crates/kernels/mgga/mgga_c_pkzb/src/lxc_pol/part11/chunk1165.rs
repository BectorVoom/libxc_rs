//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1165/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1165<F: Float>(t3069: F, t3769: F, t6142: F, t11264: F, t18617: F, t851: F, t11261: F, t2197: F, t11260: F, t2242: F, t2240: F, t6199: F, t9867: F, t11233: F, t18589: F, t18592: F) -> (F, F, F, F, F, F) {
    let t31394 = 0.28947563097646563121e3 * t6142 * t3769 * t3069;
    let t31397 = 0.62071215503128080361e4 * t18617 * t11264 * t851;
    let t31400 = 2.0 * t2197 * t11261 * t851;
    let t31401 = t11260 * t2242;
    let t31404 = 0.16081979498692535067e2 * t2240 * t31401 * t851;
    let t31407 = 0.1551780387578202009e4 * t6199 * t9867 * t3069;
    let t31411 = 0.24955700379505800916e5 * t18589 * t11233 * t18592 * t851;
    (t31394, t31397, t31400, t31404, t31407, t31411)
}
