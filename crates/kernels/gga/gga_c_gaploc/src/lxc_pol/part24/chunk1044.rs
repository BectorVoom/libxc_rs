//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1044/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1044<F: Float>(t20374: F, t7035: F, t888: F, t10296: F, t10288: F, t10286: F, t10284: F, t10282: F, t10306: F, t10285: F, t10290: F, t10304: F, t10298: F, t4349: F, t605: F, t1651: F, t3366: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31416 = t20374 * t888 * t7035;
    let t31417 = 0.76685851907841499352e0 * t31416;
    let t31447 = 12.0 * t10296;
    let t31448 = 2.0 * t10288;
    let t31449 = 4.0 * t10286;
    let t31450 = 2.0 * t10284;
    let t31451 = 2.0 * t10282;
    let t31452 = 4.0 * t10306;
    let t31453 = 2.0 * t10285;
    let t31454 = 4.0 * t10290;
    let t31455 = 2.0 * t10304;
    let t31458 = 12.0 * t4349 * t10298 * t605;
    let t31461 = 6.0 * t4349 * t3366 * t1651;
    (t31417, t31447, t31448, t31449, t31450, t31451, t31452, t31453, t31454, t31455, t31458, t31461)
}
