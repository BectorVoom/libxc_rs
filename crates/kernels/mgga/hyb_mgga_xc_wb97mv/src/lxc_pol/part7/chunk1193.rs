//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1193/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1193<F: Float>(t1427: F, t7259: F, t2595: F, t7316: F, t1415: F, t2556: F, t7333: F, t2533: F, t3541: F, t1403: F, t2517: F, t9395: F, t986: F, t2594: F, t3574: F, t1420: F, t7315: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27572 = t7259 * t1427;
    let t27575 = t2595 * t1427;
    let t27578 = t7316 * t1427;
    let t27582 = t2556 * t1415;
    let t27585 = t7333 * t1415;
    let t27591 = t3541 * t2533;
    let t27597 = t2517 * t1403;
    let t27647 = t9395 * t986;
    let t27652 = t3574 * t2594;
    let t27657 = t1420 * t7315;
    (t27572, t27575, t27578, t27582, t27585, t27591, t27597, t27647, t27652, t27657)
}
