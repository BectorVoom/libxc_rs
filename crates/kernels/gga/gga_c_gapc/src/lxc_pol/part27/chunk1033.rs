//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1033/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1033<F: Float>(t2404: F, t3439: F, t442: F, t6172: F, t2206: F, t932: F, t6851: F, t761: F, t147: F, t19: F, t2254: F, t3296: F) -> (F, F, F, F, F) {
    let t24202 = t3439 * t442 * t2404;
    let t24271 = t3439 * t6172;
    let t24352 = t932 * t2206;
    let t24398 = t761 * t6851;
    let t24499 = t3296 * t2254 * t19 * t147;
    (t24202, t24271, t24352, t24398, t24499)
}
