//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 805/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk805<F: Float>(t7375: F, t9399: F, t2660: F, t8911: F, t129: F, t8061: F, t1078: F, t8992: F, t933: F, t2600: F, t8769: F, t2629: F) -> (F, F, F, F, F, F) {
    let t9400 = t9399 * t7375;
    let t9402 = t2660 * t8911;
    let t9403 = t9402 * t7375;
    let t9405 = t8061 * t129;
    let t9406 = t9405 * t1078;
    let t9408 = t933 * t8992;
    let t9409 = t9408 * t2600;
    let t9411 = t933 * t8769;
    let t9412 = t9411 * t2629;
    (t9400, t9403, t9406, t9408, t9409, t9412)
}
