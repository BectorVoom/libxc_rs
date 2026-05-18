//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1207/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1207<F: Float>(t2389: F, t3388: F, t3750: F, t11862: F, t9425: F, t1033: F, t188: F, t2480: F, t277: F, t333: F, t311: F, t3273: F, t34081: F) -> (F, F, F, F, F) {
    let t34154 = t2389 * t3750 * t3388;
    let t34156 = t11862 * t9425;
    let t34159 = t1033 * t188;
    let t34161 = t277 * t2480 * t34159 * t333;
    let t34164 = t311 * t34081 * t3273;
    (t34154, t34156, t34159, t34161, t34164)
}
