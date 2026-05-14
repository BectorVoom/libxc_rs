//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1067/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1067<F: Float>(t4305: F, t997: F, t4278: F, t978: F, t3579: F, t3583: F, t4310: F, t6996: F, t1005: F, t4284: F, t986: F, t1422: F, t3546: F, t4300: F, t4297: F, t2562: F, t4296: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11076 = t4305 * t997;
    let t11079 = t4278 * t978;
    let t11086 = t3583 * t3579;
    let t11089 = t4310 * t6996;
    let t11090 = t11089 * t1005;
    let t11095 = t4284 * t986;
    let t11098 = t1422 * t3546;
    let t11101 = t4300 * t986;
    let t11104 = t4297 * t986;
    let t11107 = t4296 * t2562;
    (t11076, t11079, t11086, t11089, t11090, t11095, t11098, t11101, t11104, t11107)
}
