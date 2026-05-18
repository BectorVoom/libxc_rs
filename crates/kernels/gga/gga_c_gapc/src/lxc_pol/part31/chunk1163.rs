//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1163/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1163<F: Float>(t33374: F, t7595: F, t15553: F, t15555: F, t33287: F, t33158: F, t3402: F, t3408: F, t1084: F, t11428: F, t11927: F, t1461: F, t818: F) -> (F, F, F, F) {
    let t33507 = t33374 * t7595;
    let t33510 = t15553 * t33287 * t15555;
    let t33513 = t3402 * t33158 * t3408;
    let t33518 = t1084 * t1461 * t11428 * t818 * t11927;
    (t33507, t33510, t33513, t33518)
}
