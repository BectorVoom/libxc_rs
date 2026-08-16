//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 818/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk818<F: Float>(t1023: F, t9348: F, t3088: F, t3091: F, t1018: F, t1932: F, t3097: F, t197: F, t4962: F, t1022: F, t1928: F, t3096: F) -> (F, F, F, F, F, F) {
    let t9349 = t9348 * t1023;
    let t9351 = t3088 * t3091;
    let t9353 = t1932 * t1018;
    let t9354 = t9353 * t3097;
    let t9356 = t197 * t4962;
    let t9357 = t1022 * t9356;
    let t9359 = t3096 * t1928;
    (t9349, t9351, t9354, t9356, t9357, t9359)
}
