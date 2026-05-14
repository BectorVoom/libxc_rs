//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1041/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1041<F: Float>(t31379: F, t4812: F, t6583: F, t161: F, t20535: F, t4130: F, t3176: F, t493: F, t6576: F, t6578: F, t20013: F, t4782: F, t883: F, t9272: F, t20900: F, t7030: F) -> (F, F, F, F, F) {
    let t31382 = 0.38342925953920749676e1 * t6583 * t4812 * t31379;
    let t31386 = 0.23005755572352449806e1 * t20535 * t4130 * t161 * t31379;
    let t31393 = t6576 * t493 * t3176 * t6578;
    let t31394 = 0.1533717038156829987e1 * t31393;
    let t31412 = 0.11502877786176224903e1 * t9272 * t4782 * t883 * t20013;
    let t31414 = 0.59584149919750711116e-1 * t20900 * t7030;
    (t31382, t31386, t31394, t31412, t31414)
}
