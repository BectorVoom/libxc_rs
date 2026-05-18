//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1156/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1156<F: Float>(t3176: F, t493: F, t6576: F, t6578: F, t20013: F, t4782: F, t883: F, t9272: F, t20900: F, t7030: F, t20374: F, t7035: F, t888: F) -> (F, F, F, F) {
    let t31393 = t6576 * t493 * t3176 * t6578;
    let t31412 = F::new(0.11502877786176224903e1) * t9272 * t4782 * t883 * t20013;
    let t31414 = F::new(0.59584149919750711116e-1) * t20900 * t7030;
    let t31416 = t20374 * t888 * t7035;
    (t31393, t31412, t31414, t31416)
}
