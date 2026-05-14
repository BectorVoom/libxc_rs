//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 981/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk981<F: Float>(t134: F, t203: F, t5700: F, t137: F, t1672: F, t154: F, t3954: F, t26995: F, t5544: F, t1: F, t632: F, t5972: F, t3074: F, t5964: F, t1038: F, t1908: F, t1954: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27144 = t203 * t134;
    let t27145 = t27144 * t5700;
    let t27149 = t1672 * t137;
    let t27286 = t154 * t3954;
    let t27290 = t26995 * t5544;
    let t27307 = t632 * t1;
    let t27309 = t27144 * t5972;
    let t27354 = t5964 * t3074 * t5700;
    let t27408 = t1038 * t1908 * t1954;
    (t27144, t27145, t27149, t27286, t27290, t27307, t27309, t27354, t27408)
}
