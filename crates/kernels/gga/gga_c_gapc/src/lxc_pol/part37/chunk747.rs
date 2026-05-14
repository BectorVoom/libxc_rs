//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 747/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk747<F: Float>(t3133: F, t5633: F, t633: F, t8992: F, t1835: F, t1691: F, t129: F, t4948: F, t1023: F, t3088: F, t3091: F, t1018: F, t1932: F, t3097: F, t197: F, t4962: F) -> (F, F, F, F, F, F, F) {
    let t9341 = t3133 * t5633;
    let t9343 = t633 * t8992;
    let t9344 = t9343 * t1835;
    let t9346 = t9343 * t1691;
    let t9348 = t4948 * t129;
    let t9349 = t9348 * t1023;
    let t9351 = t3088 * t3091;
    let t9353 = t1932 * t1018;
    let t9354 = t9353 * t3097;
    let t9356 = t197 * t4962;
    (t9341, t9344, t9346, t9349, t9351, t9354, t9356)
}
