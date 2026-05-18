//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 526/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk526<F: Float>(t3084: F, t3085: F, t129: F, t1932: F, t1023: F, t1928: F, t197: F, t1022: F, t1018: F, t611: F) -> (F, F, F, F, F, F) {
    let t3086 = t3084 * t3085;
    let t3088 = t1932 * t129;
    let t3089 = t3088 * t1023;
    let t3091 = t197 * t1928;
    let t3092 = t1022 * t3091;
    let t3094 = t611 * t1018;
    (t3086, t3088, t3089, t3091, t3092, t3094)
}
