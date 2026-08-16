//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 326/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk326<F: Float>(t101: F, t995: F, t115: F, t594: F, t653: F, t128: F, t144: F) -> (F, F, F) {
    let t1412 = t995 * t101;
    let t1413 = t1412 * t115;
    let t1414 = t594 * t653;
    let t1415 = t1413 * t1414;
    let t1416 = t128 * t144;
    (t1412, t1415, t1416)
}
