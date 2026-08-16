//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1115/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1115<F: Float>(t5247: F, t681: F, t9261: F, t134: F, t203: F, t5700: F, t137: F, t1672: F, t154: F, t3954: F, t26995: F, t5544: F) -> (F, F, F, F, F, F) {
    let t27063 = t5247 * t681 * t9261;
    let t27144 = t203 * t134;
    let t27145 = t27144 * t5700;
    let t27149 = t1672 * t137;
    let t27286 = t154 * t3954;
    let t27290 = t26995 * t5544;
    (t27063, t27144, t27145, t27149, t27286, t27290)
}
