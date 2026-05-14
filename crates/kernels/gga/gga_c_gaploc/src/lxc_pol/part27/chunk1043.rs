//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1043/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1043<F: Float>(t20013: F, t4782: F, t883: F, t9272: F, t20900: F, t7030: F, t20374: F, t7035: F, t888: F, t10296: F, t10288: F, t10286: F, t10285: F, t10290: F, t10298: F, t4349: F, t605: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31412 = 0.11502877786176224903e1 * t9272 * t4782 * t883 * t20013;
    let t31414 = 0.59584149919750711116e-1 * t20900 * t7030;
    let t31416 = t20374 * t888 * t7035;
    let t31447 = 12.0 * t10296;
    let t31448 = 2.0 * t10288;
    let t31449 = 4.0 * t10286;
    let t31453 = 2.0 * t10285;
    let t31454 = 4.0 * t10290;
    let t31458 = 12.0 * t4349 * t10298 * t605;
    (t31412, t31414, t31416, t31447, t31448, t31449, t31453, t31454, t31458)
}
