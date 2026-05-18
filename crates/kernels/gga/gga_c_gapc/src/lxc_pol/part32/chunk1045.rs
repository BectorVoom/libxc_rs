//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1045/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1045<F: Float>(t103: F, t134: F, t22117: F, t19636: F, t647: F, t5056: F, t172: F, t6: F, t674: F, t1672: F, t3074: F, t4: F, t5972: F) -> (F, F, F, F, F) {
    let t27596 = t134 * t22117 * t103;
    let t27597 = t19636 * t647 * t27596;
    let t27622 = t5056 * M_PI;
    let t27624 = t6 * t674 * t172;
    let t27658 = t1672 * t3074 * t5972 * t4;
    (t27596, t27597, t27622, t27624, t27658)
}
