//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 926/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk926<F: Float>(t1: F, t632: F, t27144: F, t5972: F, t3074: F, t5700: F, t5964: F, t1038: F, t1908: F, t1954: F, t5059: F, t8884: F, t103: F, t134: F, t22117: F, t19636: F, t647: F) -> (F, F, F, F, F, F, F) {
    let t27307 = t632 * t1;
    let t27309 = t27144 * t5972;
    let t27354 = t5964 * t3074 * t5700;
    let t27408 = t1038 * t1908 * t1954;
    let t27420 = t8884 * t5059;
    let t27596 = t134 * t22117 * t103;
    let t27597 = t19636 * t647 * t27596;
    (t27307, t27309, t27354, t27408, t27420, t27596, t27597)
}
