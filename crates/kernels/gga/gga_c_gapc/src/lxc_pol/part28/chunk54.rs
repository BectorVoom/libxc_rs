//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 54/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk54<F: Float>(t128: F, t137: F, t122: F, t124: F, t135: F) -> (F, F, F, F, F) {
    let t138 = t128 * t137;
    let t139 = F::new(1.0) / t122;
    let t140 = t139 * t124;
    let t141 = t138 * t140;
    let t144 = F::new(30.0) + F::cast_from(0.72806316506996704929e-2_f64) * t135 * t141;
    (t138, t139, t140, t141, t144)
}
